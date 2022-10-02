// lib.rs
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386],
        color: [0.5, 0.0, 0.5, 1.],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647],
        color: [0.5, 0.0, 0.5, 1.],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706],
        color: [0.5, 0.0, 0.5, 1.],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291],
        color: [0.5, 0.0, 0.5, 1.],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359],
        color: [0.5, 0.0, 0.5, 1.],
    }, // E
];

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
