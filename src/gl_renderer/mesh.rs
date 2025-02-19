use glam::Vec3;

pub struct Mesh {
    pub vertices: Vec<MeshVertex>,
    pub triangles: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn get_flat_vertices(&self) -> (Vec<f32>, Vec<u8>, Vec<f32>) {
        let vertex_count = self.triangles.len() * 3;
        let mut positions: Vec<f32> = Vec::with_capacity(vertex_count * 3);
        let mut colors: Vec<u8> = Vec::with_capacity(vertex_count * 4);
        let mut normals: Vec<f32> = Vec::with_capacity(vertex_count * 3);

        for indices in &self.triangles {
            for i in indices {
                let v = &self.vertices[*i];
                positions.extend_from_slice(&v.position.to_array());
                colors.extend_from_slice(&v.color.to_array());
                normals.extend_from_slice(&v.normal.to_array());
            }
        }

        (positions, colors, normals)
    }
}

pub struct MeshVertex {
    pub position: Vec3,
    pub color: Color4,
    pub normal: Vec3,
}

pub struct Color4 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color4 {
    pub fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
