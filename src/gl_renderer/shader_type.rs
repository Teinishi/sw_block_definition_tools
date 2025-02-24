use eframe::{egui_glow, glow};
use enum_map::{enum_map, Enum, EnumMap};

use super::GlConfig;

const BASIC_SHADER_SOURCES: [(u32, &str); 2] = [
    (glow::VERTEX_SHADER, include_str!("./shaders/basic.vert")),
    (glow::FRAGMENT_SHADER, include_str!("./shaders/basic.frag")),
];

const GLASS_SHADER_SOURCES: [(u32, &str); 2] = [
    (glow::VERTEX_SHADER, include_str!("./shaders/glass.vert")),
    (glow::FRAGMENT_SHADER, include_str!("./shaders/glass.frag")),
];

const LINE_SHADER_SOURCES: [(u32, &str); 2] = [
    (glow::VERTEX_SHADER, include_str!("./shaders/line.vert")),
    (glow::FRAGMENT_SHADER, include_str!("./shaders/line.frag")),
];

#[derive(Debug, Enum, Clone, Copy, PartialEq)]
pub enum ShaderType {
    Basic,
    Glass,
    Line,
}

impl ShaderType {
    pub fn is_translucent(self) -> bool {
        self == Self::Glass
    }

    pub fn create_program(&self, gl: &glow::Context) -> Option<glow::Program> {
        use glow::HasContext as _;

        let shader_sources = match self {
            Self::Basic => BASIC_SHADER_SOURCES,
            Self::Glass => GLASS_SHADER_SOURCES,
            Self::Line => LINE_SHADER_SOURCES,
        };

        let shader_version = egui_glow::ShaderVersion::get(gl);
        if !shader_version.is_new_shader_interface() {
            log::warn!(
                "Custom 3D painting hasn't been ported to {:?}",
                shader_version
            );
            return None;
        }

        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let mut shaders = Vec::with_capacity(shader_sources.len());
            for (shader_type, shader_source) in shader_sources.iter() {
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
    }

    pub fn create_programs(gl: &glow::Context) -> EnumMap<Self, glow::Program> {
        enum_map! {
            Self::Basic => Self::Basic.create_program(gl).expect("Failed to create shader program"),
            Self::Glass => Self::Glass.create_program(gl).expect("Failed to create shader program"),
            Self::Line => Self::Line.create_program(gl).expect("Failed to create shader program"),
        }
    }
}

#[derive(Default, Debug)]
pub struct ShaderAttributeData {
    pub positions: Option<Vec<f32>>,
    pub colors: Option<Vec<f32>>,
    pub normals: Option<Vec<f32>>,
}

impl ShaderAttributeData {
    pub fn vertex_count(&self) -> Option<usize> {
        Some(
            self.positions
                .clone()
                .or(self.colors.clone())
                .or(self.normals.clone())?
                .len(),
        )
    }
}

impl<'a> IntoIterator for &'a ShaderAttributeData {
    type Item = (&'a str, i32, &'a Vec<f32>);
    type IntoIter = ShaderAttributeDataIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            data: self,
            index: 0,
        }
    }
}

pub struct ShaderAttributeDataIter<'a> {
    data: &'a ShaderAttributeData,
    index: usize,
}

impl<'a> Iterator for ShaderAttributeDataIter<'a> {
    type Item = (&'a str, i32, &'a Vec<f32>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut val = None;
        if self.index == 0 {
            if let Some(positions) = &self.data.positions {
                val = Some(("vertex_position_in", 3, positions));
            }
            self.index += 1;
        }
        if val.is_none() && self.index == 1 {
            if let Some(colors) = &self.data.colors {
                val = Some(("vertex_color_in", 4, colors));
            }
            self.index += 1;
        }
        if val.is_none() && self.index == 2 {
            if let Some(normals) = &self.data.normals {
                val = Some(("vertex_normal_in", 3, normals));
            }
            self.index += 1;
        }
        val
    }
}

pub trait SceneObjectContent: Send + Sync {
    fn get_shader_attribute_data(&self) -> ShaderAttributeData;
    fn gl_config(&self) -> GlConfig;
    fn center(&self) -> glam::Vec3;
}
