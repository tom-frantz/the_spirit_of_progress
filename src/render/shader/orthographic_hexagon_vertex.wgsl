// Vertex shader

struct View {
    view_proj: mat4x4<f32>,
    inverse_view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    inverse_view: mat4x4<f32>,
    projection: mat4x4<f32>,
    inverse_projection: mat4x4<f32>,
    world_position: vec3<f32>,
    width: f32,
    height: f32,
};
@group(0) @binding(0)
var<uniform> view: View;

// I nicked this from bevy_ecs_tilemap and it doesn't work :sob:
//#import bevy_sprite::mesh2d_view_bindings

struct Mesh {
    model: mat4x4<f32>,
    size: f32,
};
@group(1) @binding(0)
var<uniform> mesh: Mesh;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;

    var x = sin((model.position.x / 180.0) * 3.14159274 * 0.5) * cos((model.position.y / 90.0) * 3.14159274 * 0.5);
    var y = sin((model.position.y / 90.0) * 3.14159274 * 0.5);
    var z = cos((model.position.x / 180.0) * 3.14159274 * 0.5) * cos((model.position.y / 90.0) * 3.14159274 * 0.5);

    x = x * mesh.size / 8.0;
    y = y * mesh.size / 8.0;
    z = z * mesh.size / 8.0;

//    var x_adj = 50.0 / view.width;
//    var y_adj = 50.0 / 200.0;


    out.clip_position = vec4<f32>(x, y, z, 1.0);
    out.clip_position = mesh.model * out.clip_position;
    out.clip_position = view.view_proj * out.clip_position;
    return out;
}
