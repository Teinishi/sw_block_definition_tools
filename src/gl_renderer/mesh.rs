use super::{Color4, GetShaderAttributeData, GlConfig, ShaderAttributeData, ShaderType};
use eframe::glow;
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

    fn gl_config(&self) -> super::GlConfig {
        GlConfig {
            shader_type: ShaderType::Basic,
            mode: glow::TRIANGLES,
            line_width: None,
        }
    }
}

impl Mesh {
    pub fn signle_color_lh(
        positions: Vec<Vec3>,
        triangles: Vec<[usize; 3]>,
        color: Color4,
    ) -> Self {
        let mut vertices = Vec::with_capacity(triangles.len() * 3);
        let mut new_triangles = Vec::with_capacity(triangles.len());

        for indices in triangles {
            let i0 = vertices.len();

            let p0 = positions[indices[0]];
            let p0 = Vec3::new(p0.x, p0.y, -p0.z);
            let p1 = positions[indices[1]];
            let p1 = Vec3::new(p1.x, p1.y, -p1.z);
            let p2 = positions[indices[2]];
            let p2 = Vec3::new(p2.x, p2.y, -p2.z);

            let normal = (p1 - p0).cross(p2 - p0).normalize();
            for position in [p0, p1, p2] {
                vertices.push(MeshVertex {
                    position,
                    color,
                    normal,
                });
            }
            new_triangles.push([i0, i0 + 1, i0 + 2]);
        }

        Self {
            vertices,
            triangles: new_triangles,
        }
    }
}

#[derive(Debug)]
pub struct MeshVertex {
    pub position: Vec3,
    pub color: Color4,
    pub normal: Vec3,
}
