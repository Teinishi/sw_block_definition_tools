use super::{Line, Mesh, SceneObjectContent};
use glam::Mat4;

#[derive(Default)]
pub struct Scene {
    objects: Vec<SceneObject>,
    is_changed: bool,
}

impl Scene {
    pub fn paint(&mut self) -> Option<&Self> {
        if self.is_changed {
            self.is_changed = false;
            Some(self)
        } else {
            None
        }
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

pub struct SceneObject {
    pub contents: Vec<Box<dyn SceneObjectContent>>,
    pub transform_matrix: Mat4,
    pub always_top: bool,
    pub z_offset: f32,
}

impl SceneObject {
    pub fn from_mesh(mesh: Mesh, transform_matrix: Option<Mat4>) -> Self {
        Self {
            contents: mesh
                .submeshes
                .iter()
                .map(|submesh| Box::new(submesh.clone()) as Box<dyn SceneObjectContent>)
                .collect(),
            transform_matrix: transform_matrix.unwrap_or_default(),
            always_top: false,
            z_offset: 0.0,
        }
    }

    pub fn from_line(line: Line, transform_matrix: Option<Mat4>) -> Self {
        Self {
            contents: vec![Box::new(line)],
            transform_matrix: transform_matrix.unwrap_or_default(),
            always_top: false,
            z_offset: 0.0,
        }
    }

    pub fn set_z_offset(mut self, value: f32) -> Self {
        self.z_offset = value;
        self
    }

    pub fn apply_transform_left(mut self, transform: &Mat4) -> Self {
        self.transform_matrix = transform.mul_mat4(&self.transform_matrix);
        self
    }
}
