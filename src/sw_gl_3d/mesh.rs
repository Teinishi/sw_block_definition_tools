use super::{GlConfig, SwMesh, SwSubmesh};
use crate::sw_gl_3d::{Color4, SceneObjectContent, ShaderAttributeData, ShaderType};
use glam::Vec3;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Mesh {
    pub submeshes: Vec<Submesh>,
}

impl Mesh {
    pub fn from_submeshes(submeshes: Vec<Submesh>) -> Self {
        Self { submeshes }
    }

    pub fn from_sw_mesh(sw_mesh: &SwMesh) -> Self {
        let submeshes = sw_mesh
            .submeshes
            .iter()
            .map(|sw_submesh| Submesh::from_sw_submesh(sw_mesh, sw_submesh))
            .collect();
        Self { submeshes }
    }

    pub fn single_face_lh(positions: Vec<Vec3>, color: Color4) -> Self {
        Self::from_submeshes(vec![Submesh::single_face_lh(positions, color)])
    }

    pub fn single_color_lh(positions: Vec<Vec3>, polygons: Vec<&[usize]>, color: Color4) -> Self {
        Self::from_submeshes(vec![Submesh::single_color_lh(positions, polygons, color)])
    }

    pub fn multiple_color_lh(
        positions: Vec<Vec3>,
        faces_colors: Vec<(Vec<&[usize]>, Color4)>,
    ) -> Self {
        Self::from_submeshes(vec![Submesh::multiple_color_lh(positions, faces_colors)])
    }
}

#[derive(Debug, Clone)]
pub struct Submesh {
    vertices: Vec<MeshVertex>,
    triangles: Vec<[usize; 3]>,
    material: MeshMaterial,
    center: Vec3,
}

impl Submesh {
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

    pub fn from_sw_submesh(sw_mesh: &SwMesh, sw_submesh: &SwSubmesh) -> Self {
        let index_buffer_range = sw_submesh.index_buffer_range();
        let len_triangles = index_buffer_range.len();

        let mut vertices = Vec::with_capacity(len_triangles * 3);
        let mut triangles = Vec::with_capacity(len_triangles);

        for triangle_index in index_buffer_range {
            let indices = &sw_mesh.triangles[triangle_index as usize].as_usize_arr();
            let vertex_index = vertices.len();
            for i in indices {
                vertices.push(sw_mesh.vertices[*i].as_mesh_vertex());
            }
            triangles.push([vertex_index, vertex_index + 1, vertex_index + 2]);
        }

        let submesh = Self::new(vertices, triangles);
        match sw_submesh.shader_id {
            1 => submesh.glass(),
            2 => submesh.additive(),
            _ => submesh,
        }
    }
}

impl SceneObjectContent for Submesh {
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

impl Submesh {
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

#[derive(Debug, Clone)]
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
