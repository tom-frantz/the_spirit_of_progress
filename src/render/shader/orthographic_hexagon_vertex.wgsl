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

    out.clip_position = vec4<f32>(x, y, 0.1, 1.0);
    return out;
}
