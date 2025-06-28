use crate::{
    definition_hub::{BlockDefinition, DefinitionRegistory},
    state::State,
    sw_gl_3d::{OrbitCamera, SceneRenderer},
    ui::{paint_canvas_3d, BlockViewScene, BlockViewStateMeshOptions},
};
use egui::{CentralPanel, ScrollArea, TopBottomPanel};
use glam::Vec3;
use std::sync::Arc;

fn default_camera() -> OrbitCamera {
    OrbitCamera::new(
        Vec3::ZERO,
        Vec3::new(1.0, -0.5, 1.0),
        45f32.to_radians(),
        1.0,
    )
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Definition3dPanel {
    #[serde(skip)]
    scene: BlockViewScene,
    camera: OrbitCamera,
    #[serde(skip)]
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    #[serde(skip)]
    mesh_loaded: bool,
    #[serde(skip)]
    scene_update_done: bool,
}

impl Definition3dPanel {
    pub fn new(camera: Option<OrbitCamera>) -> Self {
        let camera = camera.unwrap_or_else(default_camera);

        Self {
            scene: Default::default(),
            camera,
            renderer: None,
            mesh_loaded: false,
            scene_update_done: false,
        }
    }

    pub fn creation_context<'a>(&mut self, cc: &'a eframe::CreationContext<'a>) {
        if let Some(gl) = &cc.gl {
            let renderer = SceneRenderer::new(gl, self.scene.scene());
            self.renderer = Some(Arc::new(egui::mutex::Mutex::new(renderer)));
        }
    }

    pub fn destroy(&self, gl: Option<&eframe::glow::Context>) {
        if let Some(renderer) = &self.renderer {
            renderer.lock().destroy(gl);
        }
    }

    pub fn reset(&mut self) {
        self.scene.reset();
        self.camera = default_camera();
        self.mesh_loaded = false;
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        state: &State,
        registory: &mut DefinitionRegistory,
        selected: Option<&BlockDefinition>,
        select_changed: bool,
    ) {
        TopBottomPanel::bottom("definition_3d_panel_bottom")
            .default_height(250.0)
            .resizable(true)
            .show_inside(ui, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_space(4.0);

                        let mesh_loaded = selected
                            .map(|d| d.load_meshes(state).borrow().is_some())
                            .unwrap_or(false);
                        let mesh_loaded_now = mesh_loaded != self.mesh_loaded;
                        self.mesh_loaded = mesh_loaded;

                        let mesh_options = selected
                            .and_then(|d| {
                                let (data, meshes) = d.load_data_meshes(state);
                                let x = meshes.borrow().as_ref().map(|meshes| {
                                    BlockViewStateMeshOptions::from_definition_meshes(meshes, &data)
                                });
                                x
                            })
                            .unwrap_or_default();

                        let scene_state_changed = self.scene.state_ui(ui, &mesh_options);
                        if !self.scene_update_done
                            || mesh_loaded_now
                            || select_changed
                            || scene_state_changed
                        {
                            if let Some(definition) = &selected {
                                self.scene_update_done =
                                    self.scene.update(definition, registory, state);
                            }
                        }

                        ui.add_space(4.0);
                    });
            });

        CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .inner_margin(0.0)
                    .fill(egui::Color32::TRANSPARENT),
            )
            .show_inside(ui, |ui| {
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
                self.camera.set_aspect_ratio(rect.width() / rect.height());
                self.camera.control(ui, response, true, true, true);
                if let Some(renderer) = &self.renderer {
                    paint_canvas_3d(
                        ui,
                        rect,
                        self.camera.clone(),
                        renderer.clone(),
                        &Default::default(),
                    );
                }
            });
    }
}
