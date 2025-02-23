use super::{Camera, Scene, ShaderType};
use eframe::glow::{self, HasContext};
use egui::mutex::Mutex;
use enum_map::EnumMap;
use glam::{Mat4, Vec3, Vec4};
use std::sync::Arc;

const SKY_COLOR_UP: Vec3 = Vec3 {
    x: 0.0,
    y: 61.0 / 255.0,
    z: 182.0 / 255.0,
};

const SKY_COLOR_DOWN: Vec3 = Vec3 {
    x: 139.0 / 255.0,
    y: 210.0 / 255.0,
    z: 207.0 / 255.0,
};

pub struct SceneRenderer {
    programs: EnumMap<ShaderType, glow::Program>,
    vaos: Vec<VaoContainer>,
    render_error: Option<String>,
    scene: Arc<Mutex<Scene>>,
}

#[allow(unsafe_code)]
impl SceneRenderer {
    pub fn new(gl: &glow::Context, scene: Arc<Mutex<Scene>>) -> Self {
        Self {
            programs: ShaderType::create_programs(gl),
            vaos: Vec::new(),
            render_error: None,
            scene,
        }
    }

    pub fn destroy(&self, gl: Option<&eframe::glow::Context>) {
        use glow::HasContext as _;

        if let Some(gl) = gl {
            unsafe {
                for (_, program) in &self.programs {
                    gl.delete_program(*program);
                }
                for vao_container in &self.vaos {
                    gl.delete_vertex_array(vao_container.vao);
                }
            }
        }
    }

    pub fn paint(&mut self, gl: &glow::Context, camera: Arc<Mutex<impl Camera>>) {
        use glow::HasContext as _;

        let override_color_1 = Vec4::ONE;
        let override_color_2 = Vec4::ONE;
        let override_color_3 = Vec4::ONE;

        let mat_view_proj = camera.lock().mat_view_proj();
        let camera_position = camera.lock().position();

        if let Some(scene) = self.scene.lock().paint() {
            match update_vaos(&self.programs, gl, scene) {
                Ok(vaos) => self.vaos = vaos,
                Err(mes) => self.render_error = Some(mes),
            }
        }

        let mut vaos: Vec<(bool, &VaoContainer)> = self
            .vaos
            .iter()
            .map(|vao_container| {
                (
                    vao_container.config.shader_type.is_translucent(),
                    vao_container,
                )
            })
            .collect();
        // 不透明オブジェクトを先に、半透明オブジェクトはカメラから遠い順に描画
        // 同一VAO内で重なっていた場合はどうにもならんが、そうなることはたぶんない
        vaos.sort_by(|a, b| {
            a.0.cmp(&b.0).then_with(|| {
                let da = (a.1.center - camera_position).length();
                let db = (b.1.center - camera_position).length();
                db.partial_cmp(&da).unwrap_or(std::cmp::Ordering::Greater)
            })
        });

        unsafe {
            gl.clear(glow::DEPTH_BUFFER_BIT);

            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LESS);

            gl.enable(glow::CULL_FACE);
            gl.cull_face(glow::BACK);
            gl.front_face(glow::CCW);

            #[cfg(not(target_arch = "wasm32"))]
            gl.enable(glow::MULTISAMPLE);

            for (_, vao_container) in vaos {
                let program = self.programs[vao_container.config.shader_type];

                gl.use_program(Some(program));

                if let Some(line_width) = vao_container.config.line_width {
                    gl.line_width(line_width);
                }

                set_uniform_mat4(gl, program, "mat_view_proj", mat_view_proj);
                set_uniform_mat4(gl, program, "mat_view_proj_inverse", mat_view_proj);
                set_uniform_mat4(gl, program, "mat_world", vao_container.transform);
                set_uniform_vec4(gl, program, "override_color_1", override_color_1);
                set_uniform_vec4(gl, program, "override_color_2", override_color_2);
                set_uniform_vec4(gl, program, "override_color_3", override_color_3);
                set_uniform_i32(gl, program, "is_preview", 1);

                set_uniform_vec3(gl, program, "camera_position", camera_position);
                set_uniform_vec4(
                    gl,
                    program,
                    "glass_color",
                    Vec4::new(0.627451, 0.627451, 0.78039217, 0.5019608),
                );
                set_uniform_vec3(gl, program, "sky_color_up", SKY_COLOR_UP);
                set_uniform_vec3(gl, program, "sky_color_down", SKY_COLOR_DOWN);

                gl.bind_vertex_array(Some(vao_container.vao));
                gl.draw_arrays(vao_container.config.mode, 0, vao_container.vertex_count);
            }
        }
    }
}

fn update_vaos(
    programs: &EnumMap<ShaderType, glow::Program>,
    gl: &glow::Context,
    scene: &Scene,
) -> Result<Vec<VaoContainer>, String> {
    scene
        .objects()
        .iter()
        .map(|object| {
            let config = object.gl_config();
            let (vao, vertex_count) =
                object.create_vertex_buffer(gl, &programs[config.shader_type])?;
            Ok(VaoContainer {
                vao,
                transform: *object.transform_matrix(),
                vertex_count: vertex_count as i32,
                config,
                center: object.center(),
            })
        })
        .collect()
}

#[derive(Debug)]
pub struct GlConfig {
    pub shader_type: ShaderType,
    pub mode: u32,
    pub line_width: Option<f32>,
}

#[derive(Debug)]
struct VaoContainer {
    vao: glow::VertexArray,
    transform: Mat4,
    vertex_count: i32,
    config: GlConfig,
    center: Vec3,
}

unsafe fn set_uniform_vec3(gl: &glow::Context, program: glow::Program, name: &str, value: Vec3) {
    gl.uniform_3_f32(
        gl.get_uniform_location(program, name).as_ref(),
        value.x,
        value.y,
        value.z,
    );
}

unsafe fn set_uniform_vec4(gl: &glow::Context, program: glow::Program, name: &str, value: Vec4) {
    gl.uniform_4_f32(
        gl.get_uniform_location(program, name).as_ref(),
        value.x,
        value.y,
        value.z,
        value.w,
    );
}

unsafe fn set_uniform_mat4(gl: &glow::Context, program: glow::Program, name: &str, value: Mat4) {
    gl.uniform_matrix_4_f32_slice(
        gl.get_uniform_location(program, name).as_ref(),
        false,
        &value.to_cols_array(),
    );
}

unsafe fn set_uniform_i32(gl: &glow::Context, program: glow::Program, name: &str, value: i32) {
    gl.uniform_1_i32(gl.get_uniform_location(program, name).as_ref(), value);
}
