use super::State;
use crate::gl_renderer::{Scene, SceneObject, SceneRenderer};
use eframe::egui_glow;
use egui::vec2;
use std::sync::Arc;

pub struct Definition3dPanel {
    scene: Scene,
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    //framebuffer: Option<MultisampleFramebuffer>,
}

impl Definition3dPanel {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let gl = cc.gl.as_ref()?;
        let scene = Scene::default();
        let renderer = SceneRenderer::new(gl.clone());

        Some(Self {
            scene,
            renderer: Some(Arc::new(egui::mutex::Mutex::new(renderer))),
            //framebuffer: MultisampleFramebuffer::new(gl.clone(), 512, 512, 16),
        })
    }

    pub fn destroy(&self) {
        if let Some(renderer) = &self.renderer {
            renderer.lock().destroy();
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
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

            // TODO
            self.update_scene(state);
        }
    }

    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_width();
        let (rect, _response) = ui.allocate_exact_size(vec2(size, size), egui::Sense::drag());

        if let Some(renderer) = self.renderer.clone() {
            let cb = egui_glow::CallbackFn::new(move |_info, painter| {
                renderer.lock().paint(painter.gl());
            });

            let callback = egui::PaintCallback {
                rect,
                callback: Arc::new(cb),
            };
            ui.painter().add(callback);
        }
    }

    fn update_scene(&mut self, state: &mut State) {
        self.scene.clear();
        if let Some(definition) = state.selected_definition() {
            let meshes = definition.meshes();

            for (key, show) in state.show_mesh() {
                if !*show {
                    continue;
                }
                if let Some(mesh) = meshes.get_mesh(&key) {
                    if let Ok(mesh) = mesh {
                        self.scene.add_object(SceneObject::new(mesh.into_mesh()));
                    }
                }
            }
        }
        if let Some(renderer) = &self.renderer {
            if let Err(mes) = renderer.lock().update_vertex_buffer(&self.scene) {
                println!("update vertex buffer error: {mes}");
            }
        }
    }
}
