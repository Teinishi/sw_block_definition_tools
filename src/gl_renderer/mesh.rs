use super::{Color4, GetShaderAttributeData, ShaderAttributeData, ShaderType};
use glam::Vec3;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<MeshVertex>,
    pub triangles: Vec<[usize; 3]>,
}

impl GetShaderAttributeData for Mesh {
    fn get_shader_attribute_data(&self) -> ShaderAttributeData {
        let vertex_count = self.triangles.len() * 3;
        let mut positions: Vec<f32> = Vec::with_capacity(vertex_count * 3);
        let mut colors: Vec<f32> = Vec::with_capacity(vertex_count * 4);
        let mut normals: Vec<f32> = Vec::with_capacity(vertex_count * 3);

        for indices in &self.triangles {
            for i in indices {
                let v = &self.vertices[*i];
                positions.extend_from_slice(&v.position.to_array());
                colors.extend_from_slice(&v.color.as_array());
                normals.extend_from_slice(&v.normal.to_array());
            }
        }

        ShaderAttributeData {
            positions: Some(positions),
            colors: Some(colors),
            normals: Some(normals),
        }
    }

    fn shader_type(&self) -> ShaderType {
        ShaderType::Basic
    }
}

#[derive(Debug)]
pub struct MeshVertex {
    pub position: Vec3,
    pub color: Color4,
    pub normal: Vec3,
}
