// Vertex shader

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
//    var x = sin(model.position.x * 3.14159274 * 0.5) * cos(model.position.y * 3.14159274 * 0.5);
//    var y = sin(model.position.y * 3.14159274 * 0.5) * cos(model.positon.x * 3.14159274 * 0.5);
    var x = sin(model.position.x * 3.14159274 * 0.5) * cos(model.position.y * 3.14159274 * 0.5);
    var y = sin(model.position.y * 3.14159274 * 0.5);

    out.clip_position = vec4<f32>(x, y, 0.1, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}