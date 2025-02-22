use super::State;
use crate::gl_renderer::{Color4, Lines, OrbitCamera, Scene, SceneObject, SceneRenderer};
use eframe::egui_glow;
use egui::{mutex::Mutex, vec2};
use glam::{vec3, Vec3};
use std::sync::Arc;

pub struct Definition3dPanel {
    scene: Arc<Mutex<Scene>>,
    camera: Arc<Mutex<OrbitCamera>>,
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    //framebuffer: Option<MultisampleFramebuffer>,
}

impl Definition3dPanel {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let gl = cc.gl.as_ref()?;
        let scene = Arc::new(Mutex::new(Scene::default()));
        let camera = Arc::new(Mutex::new(OrbitCamera {
            direction: Vec3::new(-0.5, -1.0, -2.0),
            ..Default::default()
        }));
        let renderer = SceneRenderer::new(gl, scene.clone());

        Some(Self {
            scene: scene.clone(),
            camera,
            renderer: Some(Arc::new(egui::mutex::Mutex::new(renderer))),
            //framebuffer: MultisampleFramebuffer::new(gl.clone(), 512, 512, 16),
        })
    }

    pub fn destroy(&self, gl: Option<&eframe::glow::Context>) {
        if let Some(renderer) = &self.renderer {
            renderer.lock().destroy(gl);
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        egui::Frame::canvas(ui.style())
            .fill(egui::Color32::TRANSPARENT)
            .show(ui, |ui| {
                self.custom_painting(ui);
            });

        if let Some(definition) = state.selected_definition() {
            let meshes = definition.meshes();

            let mut change = None;
            for (key, show) in state.show_mesh() {
                if let Some(mesh) = meshes.get_mesh(&key) {
                    let name = key.xml_name();
                    if let Err(err) = mesh {
                        ui.collapsing(format!("{}: Error", name), |ui| {
                            ui.label(format!("{}", err));
                        });
                    } else {
                        let mut c = *show;
                        ui.checkbox(&mut c, name);
                        if c != *show {
                            change = Some((key, c));
                        }
                    }
                }
            }
            if let Some((key, value)) = change {
                state.set_show_mesh(key, value);
            }

            if state.is_changed() {
                self.update_scene(state);
            }
        }
    }

    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_width();
        let (rect, response) = ui.allocate_exact_size(vec2(size, size), egui::Sense::drag());

        self.camera.lock().control(ui, response);
        let camera = self.camera.clone();

        if let Some(renderer) = self.renderer.clone() {
            let cb = egui_glow::CallbackFn::new(move |_info, painter| {
                renderer.lock().paint(painter.gl(), camera.clone());
            });

            let callback = egui::PaintCallback {
                rect,
                callback: Arc::new(cb),
            };
            ui.painter().add(callback);
        }
    }

    fn update_scene(&mut self, state: &mut State) {
        self.scene.lock().clear();

        if let Some(definition) = state.selected_definition() {
            let meshes = definition.meshes();

            for (key, show) in state.show_mesh() {
                if !*show {
                    continue;
                }
                if let Some(Ok(mesh)) = meshes.get_mesh(&key) {
                    self.scene
                        .lock()
                        .add_object(SceneObject::from_mesh(mesh.as_mesh(), None));
                }
            }
        }

        for (direction, color) in [
            (Vec3::X, Color4::RED),
            (Vec3::Y, Color4::GREEN),
            (Vec3::Z, Color4::BLUE),
        ] {
            self.scene.lock().add_object(SceneObject::from_line(
                Lines::single_color(vec![vec3(0.0, 0.0, 0.0), 100.0 * direction], color, 2.0),
                None,
            ));
        }
    }
}
