use super::{Color4, GlConfig, SceneObjectContent, ShaderAttributeData, ShaderType};
use eframe::glow;
use glam::Vec3;

#[derive(Debug)]
pub struct Line {
    vertices: Vec<LineVertex>,
    line_width: f32,
    center: Vec3,
}

impl Line {
    pub fn new(vertices: Vec<LineVertex>, line_width: f32) -> Self {
        let center =
            vertices.iter().fold(Vec3::ZERO, |a, b| a + b.position) / (vertices.len() as f32);
        Self {
            vertices,
            line_width,
            center,
        }
    }

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
        Self::new(vertices, line_width)
    }
}

impl SceneObjectContent for Line {
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

    fn center(&self) -> Vec3 {
        self.center
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
