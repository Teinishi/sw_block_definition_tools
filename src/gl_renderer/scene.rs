use super::{GlConfig, Line, Mesh, SceneObjectContent};
use eframe::glow;
use glam::{Mat4, Vec3};

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
    content: Box<dyn SceneObjectContent>,
    transform_matrix: Mat4,
}

impl SceneObject {
    pub fn from_mesh(mesh: Mesh, transform_matrix: Option<Mat4>) -> Self {
        Self {
            content: Box::new(mesh),
            transform_matrix: transform_matrix.unwrap_or_default(),
        }
    }

    pub fn from_line(line: Line, transform_matrix: Option<Mat4>) -> Self {
        Self {
            content: Box::new(line),
            transform_matrix: transform_matrix.unwrap_or_default(),
        }
    }

    pub fn transform_matrix(&self) -> &Mat4 {
        &self.transform_matrix
    }

    pub fn gl_config(&self) -> GlConfig {
        self.content.gl_config()
    }

    pub fn center(&self) -> Vec3 {
        self.content.center()
    }

    pub fn create_vertex_buffer(
        &self,
        gl: &glow::Context,
        program: &glow::Program,
    ) -> Result<(glow::VertexArray, usize), String> {
        use glow::HasContext as _;

        let attribute_data = self.content.get_shader_attribute_data();

        unsafe {
            let vao = gl.create_vertex_array()?;
            gl.bind_vertex_array(Some(vao));

            for (name, size, data) in &attribute_data {
                if let Some(location) = gl.get_attrib_location(*program, name) {
                    let vbo = gl
                        .create_buffer()
                        .expect("Failed to create vertex buffer object");
                    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
                    gl.buffer_data_u8_slice(
                        glow::ARRAY_BUFFER,
                        to_byte_slice(data),
                        glow::STATIC_DRAW,
                    );
                    gl.enable_vertex_attrib_array(location);
                    gl.vertex_attrib_pointer_f32(location, size, glow::FLOAT, false, size * 4, 0);
                }
            }

            Ok((vao, attribute_data.vertex_count().unwrap_or(0)))
        }
    }
}

unsafe fn to_byte_slice<T>(values: &[T]) -> &[u8] {
    std::slice::from_raw_parts(values.as_ptr() as *const _, std::mem::size_of_val(values))
}
