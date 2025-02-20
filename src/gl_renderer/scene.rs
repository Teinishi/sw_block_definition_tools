use super::Mesh;
use glam::Mat4;

#[derive(Default)]
pub struct Scene {
    objects: Vec<SceneObject>,
    is_changed: bool,
}

impl Scene {
    pub fn is_changed(&self) -> bool {
        self.is_changed
    }

    pub fn add_object(&mut self, object: SceneObject) {
        self.objects.push(object);
        self.is_changed = true;
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.is_changed = true;
    }

    pub fn objects(&self) -> &Vec<SceneObject> {
        &self.objects
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
