use super::Scene;
use eframe::{
    egui_glow,
    glow::{self, HasContext},
};
use glam::{Mat4, Vec3};
use std::sync::Arc;

const VERTEX_SHADER: &str = r#"
in vec3 position;
in vec4 color;
in vec3 normal;
uniform vec3 light_dir;
uniform vec3 diffuse_color;
uniform vec3 ambient_color;
uniform mat4 mat_model;
uniform mat4 mat_view_proj;
out vec4 v_color;
void main() {
    float power = clamp(dot(normalize((mat_model * vec4(normal, 0.0)).xyz), light_dir), 0.0, 1.0);
    v_color = vec4(color.rgb * diffuse_color * power + ambient_color, color.a);
    gl_Position = mat_view_proj * mat_model * vec4(position, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
in vec4 v_color;
out vec4 color;
void main() {
    color = v_color;
}
"#;

const SHADER_SOURCES: [(u32, &str); 2] = [
    (glow::VERTEX_SHADER, VERTEX_SHADER),
    (glow::FRAGMENT_SHADER, FRAGMENT_SHADER),
];

pub struct SceneRenderer {
    gl: Arc<glow::Context>,
    program: glow::Program,
    vaos: Vec<VaoContainer>,
}

#[allow(unsafe_code)]
impl SceneRenderer {
    pub fn new(gl: Arc<glow::Context>) -> Self {
        unsafe {
            let program = create_program(&gl).unwrap();

            Self {
                gl,
                program,
                vaos: Vec::new(),
            }
        }
    }

    pub fn destroy(&self) {
        use glow::HasContext as _;
        unsafe {
            self.gl.delete_program(self.program);
            for vao_container in &self.vaos {
                self.gl.delete_vertex_array(vao_container.vao);
            }
        }
    }

    pub fn update_vertex_buffer(&mut self, scene: &Scene) -> Result<(), String> {
        unsafe {
            for vao_container in &self.vaos {
                self.gl.delete_vertex_array(vao_container.vao);
            }
            self.vaos = create_vertex_buffer(&self.gl, &self.program, scene)?;
        }
        Ok(())
    }

    pub fn paint(&self, gl: &glow::Context) {
        use glow::HasContext as _;

        let directional_light = Vec3::new(-0.5, -1.0, -0.25).normalize();
        let directional_light_color = Vec3::ONE;

        let ambient_light_color = Vec3::new(0.15, 0.15, 0.15);

        let view = Mat4::look_at_rh(Vec3::new(1.0, 0.75, -0.5), Vec3::new(0.0, 0.1, 0.0), Vec3::Y);
        let projection = Mat4::perspective_rh(60f32.to_radians(), 1.0, 0.001, 100.0);
        let mat_view_proj = projection.mul_mat4(&view);

        unsafe {
            gl.clear(glow::DEPTH_BUFFER_BIT);

            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LESS);

            gl.enable(glow::CULL_FACE);
            gl.cull_face(glow::BACK);
            gl.front_face(glow::CW);

            gl.enable(glow::MULTISAMPLE);

            gl.use_program(Some(self.program));
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "mat_view_proj").as_ref(),
                false,
                &mat_view_proj.to_cols_array(),
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "light_dir").as_ref(),
                -directional_light.x,
                -directional_light.y,
                -directional_light.z,
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "diffuse_color")
                    .as_ref(),
                directional_light_color.x,
                directional_light_color.y,
                directional_light_color.z,
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "ambient_color")
                    .as_ref(),
                ambient_light_color.x,
                ambient_light_color.y,
                ambient_light_color.z,
            );

            for vao_container in &self.vaos {
                gl.uniform_matrix_4_f32_slice(
                    gl.get_uniform_location(self.program, "mat_model").as_ref(),
                    false,
                    &vao_container.transform.to_cols_array(),
                );
                gl.bind_vertex_array(Some(vao_container.vao));
                gl.draw_arrays(glow::TRIANGLES, 0, vao_container.vertex_count);
            }
        }
    }
}

unsafe fn create_program(gl: &glow::Context) -> Option<glow::NativeProgram> {
    use glow::HasContext as _;

    let shader_version = egui_glow::ShaderVersion::get(gl);
    let program = gl.create_program().expect("Cannot create program");
    if !shader_version.is_new_shader_interface() {
        log::warn!(
            "Custom 3D painting hasn't been ported to {:?}",
            shader_version
        );
        return None;
    }

    let mut shaders = Vec::with_capacity(SHADER_SOURCES.len());
    for (shader_type, shader_source) in SHADER_SOURCES.iter() {
        let shader = gl
            .create_shader(*shader_type)
            .expect("Cannot create shader");
        gl.shader_source(
            shader,
            &format!(
                "{}\n{}",
                shader_version.version_declaration(),
                shader_source
            ),
        );
        gl.compile_shader(shader);
        assert!(
            gl.get_shader_compile_status(shader),
            "Failed to compile shader {shader_type}: {}",
            gl.get_shader_info_log(shader)
        );

        gl.attach_shader(program, shader);
        shaders.push(shader);
    }

    gl.link_program(program);
    assert!(
        gl.get_program_link_status(program),
        "{}",
        gl.get_program_info_log(program)
    );

    for shader in shaders {
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
    }

    Some(program)
}

unsafe fn create_vertex_buffer(
    gl: &glow::Context,
    program: &glow::NativeProgram,
    scene: &Scene,
) -> Result<Vec<VaoContainer>, String> {
    use glow::HasContext as _;

    let mut vaos = Vec::with_capacity(scene.object_count());

    for object in scene.objects() {
        let mesh = object.mesh();

        let (positions, colors, normals) = mesh.get_flat_vertices();
        let vertex_count = positions.len();
        let positions_u8 = to_byte_slice(&positions[..]);
        let colors_u8 = to_byte_slice(&colors[..]);
        let normals_u8 = to_byte_slice(&normals[..]);

        let attributes = [
            ("position", positions_u8, 3),
            ("color", colors_u8, 4),
            ("normal", normals_u8, 3),
        ];

        let vao = gl.create_vertex_array()?;
        gl.bind_vertex_array(Some(vao));

        for (name, data, size) in attributes {
            let attrib_position = gl.get_attrib_location(*program, name).unwrap();
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW);
            gl.enable_vertex_attrib_array(attrib_position);
            gl.vertex_attrib_pointer_f32(attrib_position, size, glow::FLOAT, false, size * 4, 0);
        }

        vaos.push(VaoContainer {
            vao,
            transform: object.transform_matrix().clone(),
            vertex_count: vertex_count as i32,
        });
    }

    Ok(vaos)
}

unsafe fn to_byte_slice<'a, T>(values: &'a [T]) -> &'a [u8] {
    std::slice::from_raw_parts(
        values.as_ptr() as *const _,
        values.len() * core::mem::size_of::<T>(),
    )
}

struct VaoContainer {
    vao: glow::NativeVertexArray,
    transform: Mat4,
    vertex_count: i32,
}
