use super::Mesh;
use glam::Mat4;

#[derive(Default)]
pub struct Scene {
    objects: Vec<SceneObject>,
}

impl Scene {
    pub fn objects(&self) -> &Vec<SceneObject> {
        &self.objects
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

pub struct SceneObject {
    mesh: Mesh,
    transform_matrix: Mat4,
}

impl SceneObject {
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn transform_matrix(&self) -> &Mat4 {
        &self.transform_matrix
    }
}
