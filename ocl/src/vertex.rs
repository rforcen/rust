#[repr(packed)]
#[derive(Copy, Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 4], // fit vec3 + pad
    pub normal: [f32; 4],
    pub color: [f32; 4],
    pub texture: [f32; 4],
}
impl Vertex {
    pub fn new() -> Self {
        Self {
            position: [0., 0., 0., 0.],
            normal: [0., 0., 0., 0.],
            color: [0., 0., 0., 0.],
            texture: [0., 0., 0., 0.],
        }
    }
}

pub fn generate_faces(n: u32) -> Vec<Vec<u32>> {
    let mut faces = vec![];
    // generate faces
    let n = n;
    for i in 0..n - 1 {
        for j in 0..n - 1 {
            faces.push(vec![
                (i + 1) * n + j,
                (i + 1) * n + j + 1,
                i * n + j + 1,
                i * n + j,
            ])
        }
        faces.push(vec![(i + 1) * n, (i + 1) * n + n - 1, i * n, i * n + n - 1]);
    }
    for i in 0..n - 1 {
        faces.push(vec![i, i + 1, n * (n - 1) + i + 1, n * (n - 1) + i])
    }
    faces
}
