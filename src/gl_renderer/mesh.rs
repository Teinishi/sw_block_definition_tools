use super::{Color4, GlConfig, SceneObjectContent, ShaderAttributeData, ShaderType};
use glam::Vec3;

#[derive(Debug)]
pub enum MeshMaterial {
    Flat,
    #[allow(dead_code)]
    Simple,
    Basic,
    Glass,
    Additive,
}

impl MeshMaterial {
    fn shader_type(&self) -> ShaderType {
        match self {
            Self::Flat => ShaderType::Flat,
            Self::Simple => ShaderType::Simple,
            Self::Basic => ShaderType::Opaque,
            Self::Glass => ShaderType::Glass,
            Self::Additive => ShaderType::Additive,
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<MeshVertex>,
    triangles: Vec<[usize; 3]>,
    material: MeshMaterial,
    center: Vec3,
}

impl Mesh {
    pub fn new(vertices: Vec<MeshVertex>, triangles: Vec<[usize; 3]>) -> Self {
        let center =
            vertices.iter().fold(Vec3::ZERO, |a, b| a + b.position) / (vertices.len() as f32);
        Self {
            vertices,
            triangles,
            material: MeshMaterial::Basic,
            center,
        }
    }
}

impl SceneObjectContent for Mesh {
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
            shader_type: self.material.shader_type(),
            mode: super::DrawArrayMode::Triangles,
            line_width: None,
        }
    }

    fn center(&self) -> Vec3 {
        self.center
    }
}

impl Mesh {
    pub fn single_face_lh(positions: Vec<Vec3>, color: Color4) -> Self {
        let polygon: Vec<usize> = (0..positions.len()).collect();
        Self::single_color_lh(positions, vec![&polygon], color)
    }

    pub fn single_color_lh(positions: Vec<Vec3>, polygons: Vec<&[usize]>, color: Color4) -> Self {
        Self::multiple_color_lh(positions, vec![(polygons, color)])
    }

    pub fn multiple_color_lh(
        positions: Vec<Vec3>,
        faces_colors: Vec<(Vec<&[usize]>, Color4)>,
    ) -> Self {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        for (polygons, color) in faces_colors {
            for indices in polygons_to_triangles(polygons) {
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
                triangles.push([i0, i0 + 1, i0 + 2]);
            }
        }

        Self::new(vertices, triangles)
    }

    pub fn combined(meshes: impl IntoIterator<Item = Self>) -> Self {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        for mut mesh in meshes {
            let offset = vertices.len();
            vertices.append(&mut mesh.vertices);
            triangles.reserve(mesh.triangles.len());
            for indices in mesh.triangles {
                triangles.push(indices.map(|i| i + offset));
            }
        }

        Self::new(vertices, triangles)
    }

    pub fn flat(mut self) -> Self {
        self.material = MeshMaterial::Flat;
        self
    }

    pub fn glass(mut self) -> Self {
        self.material = MeshMaterial::Glass;
        self
    }

    pub fn additive(mut self) -> Self {
        self.material = MeshMaterial::Additive;
        self
    }
}

#[derive(Debug)]
pub struct MeshVertex {
    pub position: Vec3,
    pub color: Color4,
    pub normal: Vec3,
}

fn polygons_to_triangles(polygons: Vec<&[usize]>) -> Vec<[usize; 3]> {
    let mut triangles = Vec::new();
    for polygon in polygons {
        let p0 = polygon[0];
        for i in 2..polygon.len() {
            let p1 = polygon[i - 1];
            let p2 = polygon[i];
            triangles.push([p0, p1, p2]);
        }
    }
    triangles
}
