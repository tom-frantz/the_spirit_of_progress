use bevy::prelude::Shader;

pub const _ORTHOGRAPHIC_HEXAGON_VERTEX_SHADER: &str = "ORTHOGRAPHIC_HEXAGON_SHADER";

pub struct HexShaders {
    pub vertex: Shader,
    pub fragment: Shader,
}

pub fn include_ortho_hex_shader() -> HexShaders {
    HexShaders {
        vertex: Shader::from_wgsl(include_str!("orthographic_hexagon_vertex.wgsl")),
        fragment: Shader::from_wgsl(include_str!("orthographic_hexagon_fragment.wgsl")),
    }
}
