use super::{Color4, GetShaderAttributeData, GlConfig, ShaderAttributeData, ShaderType};
use eframe::glow;
use glam::Vec3;

#[derive(Debug)]
pub struct Lines {
    pub vertices: Vec<LineVertex>,
    pub line_width: f32,
}

impl Lines {
    pub fn single_color(positions: Vec<Vec3>, color: Color4, line_width: f32) -> Self {
        let vertices = positions
            .iter()
            .map(|p| LineVertex {
                position: *p,
                color,
            })
            .collect();
        Self {
            vertices,
            line_width,
        }
    }
}

impl GetShaderAttributeData for Lines {
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

    fn gl_config(&self) -> GlConfig {
        GlConfig {
            shader_type: ShaderType::Line,
            mode: glow::LINES,
            line_width: Some(self.line_width),
        }
    }
}

#[derive(Debug)]
pub struct LineVertex {
    pub position: Vec3,
    pub color: Color4,
}
