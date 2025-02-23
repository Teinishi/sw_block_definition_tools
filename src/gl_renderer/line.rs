use super::{Color4, GetShaderAttributeData, GlConfig, ShaderAttributeData, ShaderType};
use eframe::glow;
use glam::Vec3;

#[derive(Debug)]
pub struct Line {
    pub vertices: Vec<LineVertex>,
    pub line_width: f32,
}

impl Line {
    pub fn single_color_lh(
        positions: Vec<Vec3>,
        color: Color4,
        line_width: f32,
        is_loop: bool,
    ) -> Self {
        let n = positions.len();
        let vertices = (0..(if is_loop { n } else { n - 1 }))
            .flat_map(|i| {
                [
                    LineVertex::from_vec3_lh(positions[i], color),
                    LineVertex::from_vec3_lh(positions[(i + 1) % n], color),
                ]
            })
            .collect();
        Self {
            vertices,
            line_width,
        }
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

impl LineVertex {
    pub fn from_vec3_lh(position: Vec3, color: Color4) -> Self {
        Self {
            position: Vec3::new(position.x, position.y, -position.z),
            color,
        }
    }
}
