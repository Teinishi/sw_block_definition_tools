use super::{Color4, GetShaderAttributeData, ShaderAttributeData, ShaderType};
use glam::Vec3;

#[derive(Debug)]
pub struct Line {
    pub vertices: Vec<LineVertex>,
}

impl Line {
    pub fn single_color(positions: Vec<Vec3>, color: Color4) -> Self {
        let vertices = positions
            .iter()
            .map(|p| LineVertex {
                position: *p,
                color,
            })
            .collect();
        Self { vertices }
    }
}

impl GetShaderAttributeData for Line {
    fn get_shader_attribute_data(&self) -> ShaderAttributeData {
        let vertex_count = self.vertices.len();
        let mut positions: Vec<f32> = Vec::with_capacity(vertex_count * 3);
        let mut colors: Vec<f32> = Vec::with_capacity(vertex_count * 4);

        for v in &self.vertices {
            positions.extend_from_slice(&v.position.to_array());
            colors.extend_from_slice(&v.color.as_array());
        }

        ShaderAttributeData {
            positions: Some(positions),
            colors: Some(colors),
            normals: None,
        }
    }

    fn shader_type(&self) -> ShaderType {
        ShaderType::Line
    }
}

#[derive(Debug)]
pub struct LineVertex {
    pub position: Vec3,
    pub color: Color4,
}
