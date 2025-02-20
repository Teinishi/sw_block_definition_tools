use super::Scene;
use eframe::{
    egui_glow,
    glow::{self, HasContext},
};
use glam::{Mat4, Vec3, Vec4};
use std::sync::Arc;

/*const VERTEX_SHADER: &str = r#"
in vec3 vertexPosition_in;
in vec4 vertexColor_in;
in vec3 vertexNormal_in;
uniform vec3 light_dir;
uniform vec3 diffuse_color;
uniform vec3 ambient_color;
uniform mat4 mat_world;
uniform mat4 mat_view_proj;
out vec4 v_color;
void main() {
    float power = clamp(dot(normalize((mat_world * vec4(vertexNormal_in, 0.0)).xyz), light_dir), 0.0, 1.0);
    v_color = vec4(vertexColor_in.rgb * diffuse_color * power + ambient_color, vertexColor_in.a);
    gl_Position = mat_view_proj * mat_world * vec4(vertexPosition_in, 1.0);
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
];*/

const BASIC_VERTEX_SHADER: &str = r#"
in vec3 vertexPosition_in;
in vec4 vertexColor_in;
in vec3 vertexNormal_in;

out vec4 vertexColor_out;
out vec3 vertexNormal_out;

uniform mat4 mat_view_proj;
uniform mat4 mat_world;
uniform vec4 override_color_1;
uniform vec4 override_color_2;
uniform vec4 override_color_3;
uniform int is_preview;

void main()
{
    gl_Position =  mat_view_proj * mat_world * vec4(vertexPosition_in, 1);

    vec3 override_color_1_difference = vertexColor_in.rgb - vec3(1.0, 0.494, 0.0);
    vec3 override_color_2_difference = vertexColor_in.rgb - vec3(0.608, 0.494, 0.0);
    vec3 override_color_3_difference = vertexColor_in.rgb - vec3(0.216, 0.494, 0.0);

    vec3 surface_color_difference = vertexColor_in.rgb - vec3(1.0, 1.0, 1.0);

    if(is_preview == 1 && (dot(override_color_1_difference, override_color_1_difference) < 0.01 || dot(surface_color_difference, surface_color_difference) < 0.01))
    {
        vertexColor_out = override_color_1;
    }
    else if(is_preview == 1 && (dot(override_color_2_difference, override_color_2_difference) < 0.01 || dot(surface_color_difference, surface_color_difference) < 0.01))
    {
        vertexColor_out = override_color_2;
    }
    else if(is_preview == 1 && (dot(override_color_3_difference, override_color_3_difference) < 0.01 || dot(surface_color_difference, surface_color_difference) < 0.01))
    {
        vertexColor_out = override_color_3;
    }
    else
    {
        vertexColor_out = vertexColor_in;
    }

    vertexNormal_out = (mat_world * vec4(vertexNormal_in, 0)).xyz;
}
"#;

const BASIC_FRAGMENT_SHADER: &str = r#"
in vec4 vertexColor_out;
in vec3 vertexNormal_out;

out vec4 color_out;

void main()
{
    vec3 light_dir = vec3(0.5, -1.0, 0.2);
    float light_amount = dot(vertexNormal_out, -light_dir) * 0.4 + 0.7;
    color_out = vertexColor_out * vec4(light_amount, light_amount, light_amount, 1.0);
}
"#;

const SHADER_SOURCES: [(u32, &str); 2] = [
    (glow::VERTEX_SHADER, BASIC_VERTEX_SHADER),
    (glow::FRAGMENT_SHADER, BASIC_FRAGMENT_SHADER),
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

        let override_color_1 = Vec4::ONE;
        let override_color_2 = Vec4::ONE;
        let override_color_3 = Vec4::ONE;

        let _directional_light = Vec3::new(-0.5, -1.0, -0.25).normalize();
        let _directional_light_color = Vec3::ONE;

        let _ambient_light_color = Vec3::new(0.15, 0.15, 0.15);

        let view = Mat4::look_at_rh(
            Vec3::new(1.0, 0.75, -0.5),
            Vec3::new(0.0, 0.1, 0.0),
            Vec3::Y,
        );
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

            set_uniform_mat4(gl, self.program, "mat_view_proj", mat_view_proj);
            /*set_uniform_vec3(gl, self.program, "light_dir", -directional_light);
            set_uniform_vec3(gl, self.program, "diffuse_color", directional_light_color);
            set_uniform_vec3(gl, self.program, "ambient_color", ambient_light_color);*/

            set_uniform_color4(gl, self.program, "override_color_1", override_color_1);
            set_uniform_color4(gl, self.program, "override_color_2", override_color_2);
            set_uniform_color4(gl, self.program, "override_color_3", override_color_3);
            set_uniform_i32(gl, self.program, "is_preview", 1);

            for vao_container in &self.vaos {
                set_uniform_mat4(gl, self.program, "mat_world", vao_container.transform);
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
            ("vertexPosition_in", positions_u8, 3),
            ("vertexColor_in", colors_u8, 4),
            ("vertexNormal_in", normals_u8, 3),
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

unsafe fn _set_uniform_vec3(
    gl: &glow::Context,
    program: glow::NativeProgram,
    name: &str,
    value: Vec3,
) {
    gl.uniform_3_f32(
        gl.get_uniform_location(program, name).as_ref(),
        value.x,
        value.y,
        value.z,
    );
}
unsafe fn set_uniform_color4(
    gl: &glow::Context,
    program: glow::NativeProgram,
    name: &str,
    value: Vec4,
) {
    gl.uniform_4_f32(
        gl.get_uniform_location(program, name).as_ref(),
        value.x,
        value.y,
        value.z,
        value.w,
    );
}

unsafe fn set_uniform_mat4(
    gl: &glow::Context,
    program: glow::NativeProgram,
    name: &str,
    value: Mat4,
) {
    gl.uniform_matrix_4_f32_slice(
        gl.get_uniform_location(program, name).as_ref(),
        false,
        &value.to_cols_array(),
    );
}

unsafe fn set_uniform_i32(
    gl: &glow::Context,
    program: glow::NativeProgram,
    name: &str,
    value: i32,
) {
    gl.uniform_1_i32(gl.get_uniform_location(program, name).as_ref(), value);
}
