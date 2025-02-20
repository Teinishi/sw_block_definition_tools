use super::Mesh;
use glam::Mat4;

#[derive(Default)]
pub struct Scene {
    objects: Vec<SceneObject>,
}

impl Scene {
    pub fn add_object(&mut self, object: SceneObject) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn objects(&self) -> &Vec<SceneObject> {
        &self.objects
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

#[derive(Debug)]
pub struct SceneObject {
    mesh: Mesh,
    transform_matrix: Mat4,
}

impl SceneObject {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            transform_matrix: Mat4::IDENTITY,
        }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn transform_matrix(&self) -> &Mat4 {
        &self.transform_matrix
    }
}
