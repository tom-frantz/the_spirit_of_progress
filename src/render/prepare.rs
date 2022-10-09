use crate::render::{HexWorld, HexWorldId, Hexagon};
use bevy::{
    prelude::{Commands, Mesh, Res},
    render::{
        mesh::{
            GpuBufferInfo, GpuMesh, Indices, MeshVertexAttribute, PrimitiveTopology,
            VertexAttributeValues,
        },
        render_resource::{BufferInitDescriptor, BufferUsages, VertexFormat},
        renderer::RenderDevice,
    },
};

pub const ATTRIBUTE_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("Position", 184657321, VertexFormat::Float32x2);

pub const ATTRIBUTE_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("Color", 218479653, VertexFormat::Float32x4);

pub fn prepare(mut commands: Commands, render_device: Res<RenderDevice>) {
    let mut tiles = Vec::new();

    // Store data somewhere before it's converted for the GPU
    let mut positions: Vec<[f32; 2]> = Vec::new();
    let mut colours: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    for index in 0..=1 {
        let hex = Hexagon::from_center([0.0 + index as f32 * 0.4, 0.0]);

        for vertex in &hex.vertices {
            positions.push(*vertex);
            // println!("{vertex:?}");
            colours.push([0.8, 0.8, 0.3, 1.0])
        }

        // the `+ 5` is tied to the (temporary) iterator that starts at `-5`
        for index in hex.indices((index) as u16 * 6) {
            indices.extend(index);
        }

        tiles.push(hex);
        // commands.spawn().insert(hex);
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

    commands.insert_resource(HexWorld::new(0, gpu_mesh));
    commands.spawn().insert(HexWorldId(0));
}
