use crate::{
    game::world::{elevation::WorldElevationData, tectonics::WorldTectonicsData, HexWorldMapMode},
    render::{
        extract::{ExtractedHexWorld, ExtractedHexWorldData, ExtractedHexWorldMapMode},
        pipeline::bind_groups::MeshUniformBuffer,
        traits::QueryCellRender,
        HexWorldChunk,
    },
    ui::theme::{Colour, Terrain},
};
use bevy::{
    prelude::*,
    render::{
        mesh::{
            GpuBufferInfo, GpuMesh, Indices, MeshVertexAttribute, PrimitiveTopology,
            VertexAttributeValues,
        },
        render_resource::{BufferInitDescriptor, BufferUsages, ShaderType, VertexFormat},
        renderer::{RenderDevice, RenderQueue},
    },
};
use h3ron::{res0_cells, ToPolygon};
use std::marker::PhantomData;

#[derive(ShaderType, Component, Clone)]
pub struct MeshUniform {
    pub transform: Mat4,
    pub size: f32,
}

/// Stores the index of a uniform inside of [`ComponentUniforms`].
#[derive(Component)]
pub struct DynamicUniformIndex<C: Component> {
    index: u32,
    marker: PhantomData<C>,
}

impl<C: Component> DynamicUniformIndex<C> {
    #[inline]
    pub fn index(&self) -> u32 {
        self.index
    }
}

pub const ATTRIBUTE_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("Position", 184657321, VertexFormat::Float32x2);

pub const ATTRIBUTE_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("Color", 218479653, VertexFormat::Float32x4);

pub fn prepare(
    mut commands: Commands,

    // Uniform buffers
    mut mesh_uniforms: ResMut<MeshUniformBuffer>,

    // The GPU device that we're using
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,

    hex_world: Query<(
        &ExtractedHexWorld,
        &Transform,
        &ExtractedHexWorldMapMode,
        Option<&ExtractedHexWorldData<WorldElevationData>>,
        Option<&ExtractedHexWorldData<WorldTectonicsData>>,
    )>,
) {
    mesh_uniforms.clear();
    // For each hex world
    for (hex_world, transform, map_mode, elevation_data, tectonics_data) in hex_world.iter() {
        // Store data somewhere before it's converted for the GPU
        let mut positions: Vec<[f32; 2]> = Vec::new();
        let mut colours: Vec<[f32; 4]> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        // Find what data we have based on the map mode.
        let hex_world_data: &dyn QueryCellRender = match map_mode.0 {
            HexWorldMapMode::Elevation => elevation_data.unwrap(),
            HexWorldMapMode::Tectonics => tectonics_data.unwrap(),
        };

        let mut offset: u16 = 0;

        // TEMP: Hack to make interesting colours
        let random_colours = || {
            vec![
                Terrain::Sea6,
                Terrain::Sea5,
                Terrain::Sea4,
                Terrain::Sea3,
                Terrain::Sea2,
                Terrain::Sea1,
                Terrain::SeaLevelWater,
                Terrain::SeaLevelLand,
                Terrain::Land1,
                Terrain::Land2,
                Terrain::Land3,
                Terrain::Land4,
                Terrain::Land5,
                Terrain::Land6,
                Terrain::Land7,
                Terrain::Land8,
                Terrain::Land9,
            ]
        };
        let mut current_colours = random_colours();

        // TODO Change this to be dependent on the zoom
        for res0_cell in res0_cells().iter() {
            for cell in res0_cell.get_children(2).unwrap().iter() {
                // Get the primitive points to use.
                let poly = cell.to_polygon().expect("Should be legit lmao");
                // Skip the first: The first and last point are the same, to close the line string.
                // By skipping the first, you don't have a duplicate point/triangle in your buffer
                let line_str = poly.exterior().points().skip(1);
                let amount_of_vertices: u16 = line_str.len() as u16;

                // TEMP: Hack to make interesting colours.
                let colour = current_colours.pop();
                let colour = match colour {
                    Some(colour) => colour,
                    None => {
                        current_colours = random_colours();
                        current_colours.pop().unwrap()
                    }
                };

                for point in line_str {
                    // For each point, insert data into the buffers.

                    // Transform to local coordinates based on transform / size
                    positions.push([point.x() as f32, point.y() as f32]);

                    colours.push(colour.color().as_rgba_f32());
                    // colours.push(hex_world_data.cell_colour(cell).as_rgba_f32());
                }

                for last_vertex_index in 2..amount_of_vertices {
                    // last_vertex_index = 2: offset, offset + 1, offset + 2
                    // last_vertex_index = 3: offset, offset + 2, offset + 1
                    // etc.
                    indices.extend([
                        last_vertex_index + offset - 1,
                        0 + offset,
                        last_vertex_index + offset,
                    ])
                }

                offset += amount_of_vertices;
            }
        }
        // Create a mesh to get data from, for the render device.
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        // Insert data into the mesh, so that it can then be sent to the GPU
        mesh.insert_attribute(
            ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x2(positions),
        );
        mesh.insert_attribute(ATTRIBUTE_COLOR, VertexAttributeValues::Float32x4(colours));
        mesh.set_indices(Some(Indices::U16(indices)));

        // Create the buffers with data for the GPU
        let vertex_buffer_data = mesh.get_vertex_buffer_data();
        let vertex_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("hex_buffer"),
            contents: &vertex_buffer_data,
            usage: BufferUsages::VERTEX,
        });

        let buffer_info = mesh.get_index_buffer_bytes().map_or(
            GpuBufferInfo::NonIndexed {
                vertex_count: mesh.count_vertices() as u32,
            },
            |data| GpuBufferInfo::Indexed {
                buffer: render_device.create_buffer_with_data(&BufferInitDescriptor {
                    usage: BufferUsages::INDEX,
                    contents: data,
                    label: Some("Mesh Index Buffer"),
                }),
                count: mesh.indices().unwrap().len() as u32,
                index_format: mesh.indices().unwrap().into(),
            },
        );

        // Finalize the shape of this for the GPU. This will be used in the draw function!
        let gpu_mesh = GpuMesh {
            vertex_buffer,
            buffer_info,
            primitive_topology: PrimitiveTopology::TriangleList,
            layout: mesh.get_mesh_vertex_buffer_layout(),
        };

        commands
            .spawn()
            .insert(HexWorldChunk(0, gpu_mesh))
            .insert(DynamicUniformIndex::<MeshUniform> {
                index: mesh_uniforms.push(MeshUniform {
                    transform: transform.compute_matrix(),
                    size: hex_world.0.size() as f32,
                }),
                marker: Default::default(),
            });
    }

    mesh_uniforms.write_buffer(&render_device, &render_queue);
}

#[cfg(test)]
mod tests {
    use super::*;
    use h3ron::{res0_cells, ToPolygon};

    #[test]
    fn test() {
        for cell in res0_cells().iter() {
            // Get the primitive points to use.
            let poly = cell.to_polygon().expect("Should be legit lmao");
            let line_str = poly.exterior();
            println!("{line_str:?}\n");
        }
    }

    #[test]
    fn geometry() {
        for res0_cell in res0_cells().iter() {
            let poly = res0_cell.to_polygon().unwrap();

            let points = poly.exterior().points().skip(1);
            let number_of_vertices = points.len();

            let cleaner = points
                .map(|point| (point.x().trunc(), point.y()))
                .collect::<Vec<_>>();

            println!("{:.1$?}", cleaner, 3);
            if res0_cell.is_pentagon() {
                assert_eq!(number_of_vertices, 5)
            } else {
                assert_eq!(number_of_vertices, 6)
            }
        }
    }
}
