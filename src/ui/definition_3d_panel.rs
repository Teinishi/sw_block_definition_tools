use super::State;
use crate::gl_renderer::SceneRenderer;
use eframe::egui_glow;
use egui::vec2;
use std::sync::Arc;

pub struct Definition3dPanel {
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    //framebuffer: Option<MultisampleFramebuffer>,
}

impl Definition3dPanel {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let gl = cc.gl.as_ref()?;
        let mut renderer = SceneRenderer::new(gl.clone());
        let _ = renderer.update_vertex_buffer();

        Some(Self {
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

            let mesh_checkboxes = [
                ("mesh_data", &mut state.show_mesh_data, meshes.mesh_data()),
                ("mesh_0", &mut state.show_mesh_0, meshes.mesh_0()),
                ("mesh_1", &mut state.show_mesh_1, meshes.mesh_1()),
                ("mesh_2", &mut state.show_mesh_2, meshes.mesh_2()),
                (
                    "mesh_editor_only",
                    &mut state.show_mesh_editor_only,
                    meshes.mesh_editor_only(),
                ),
            ];

            for (name, checked, mesh) in mesh_checkboxes {
                if let Some(r) = mesh {
                    if let Err(err) = r {
                        ui.collapsing(format!("{}: Error", name), |ui| {
                            ui.label(format!("{}", err));
                        });
                    } else {
                        ui.checkbox(checked, name);
                    }
                }
            }
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
}
