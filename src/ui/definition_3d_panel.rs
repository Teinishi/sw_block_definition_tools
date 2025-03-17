use super::{paint_canvas_3d, BlockViewScene, SaveImageModal, State};
use crate::gl_renderer::{OrbitCamera, SceneRenderer};
use egui::vec2;
use glam::Vec3;
use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Definition3dPanel {
    save_image_modal: SaveImageModal,
    #[serde(skip)]
    scene: BlockViewScene,
    camera: Arc<Mutex<OrbitCamera>>,
    #[serde(skip)]
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    #[serde(skip)]
    mesh_loaded: bool,
}

impl Definition3dPanel {
    pub fn new<'a>(
        cc: &'a eframe::CreationContext<'a>,
        camera: Option<OrbitCamera>,
    ) -> Option<Self> {
        let mut camera = camera.unwrap_or_else(|| OrbitCamera {
            direction: Vec3::new(1.0, -0.5, -1.0),
            ..Default::default()
        });
        camera.orthogonalize_up();
        let camera = Arc::new(Mutex::new(camera));

        let mut instance = Self {
            save_image_modal: SaveImageModal::default(),
            scene: Default::default(),
            camera,
            renderer: None,
            mesh_loaded: false,
        };
        Self::creation_context(&mut instance, cc);
        Some(instance)
    }

    pub fn creation_context<'a>(&mut self, cc: &'a eframe::CreationContext<'a>) {
        if let Some(gl) = &cc.gl {
            let renderer = SceneRenderer::new(gl, self.scene.scene());
            self.renderer = Some(Arc::new(egui::mutex::Mutex::new(renderer)));
        }
        self.save_image_modal.creation_context(cc);
    }

    pub fn destroy(&self, gl: Option<&eframe::glow::Context>) {
        if let Some(renderer) = &self.renderer {
            renderer.lock().destroy(gl);
        }
        self.save_image_modal.destroy(gl);
    }

    #[allow(unused_variables)]
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State, frame: &eframe::Frame) {
        egui::Frame::canvas(ui.style())
            .fill(egui::Color32::TRANSPARENT)
            .inner_margin(0.0)
            .show(ui, |ui| {
                let s = ui.available_width();
                let (rect, response) = ui.allocate_exact_size(vec2(s, s), egui::Sense::drag());
                self.camera.lock().unwrap().control(ui, response);
                if let Some(renderer) = &self.renderer {
                    paint_canvas_3d(ui, rect, self.camera.clone(), renderer.clone());
                }
            });

        let (data, meshes) = if let Some(definition) = state.selected_definition() {
            (
                definition.load_data().and_then(|d| d.ok()),
                definition.load_meshes(),
            )
        } else {
            (None, None)
        };

        let mesh_loaded = meshes.is_some();
        self.scene
            .state_ui(ui, data, meshes, mesh_loaded != self.mesh_loaded);
        self.mesh_loaded = mesh_loaded;

        ui.separator();

        if ui.button("Save image").clicked() {
            self.save_image_modal.open(state.selected_definition());
        }

        self.save_image_modal
            .ui(ui, frame, &mut state.selected_definition());
    }
}
