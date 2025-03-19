use super::{
    definitions_store::DefinitionSelect, paint_canvas_3d, BlockViewScene, DefinitionSingleSelect,
    SaveImageModal,
};
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
    tracker_id: u32,
}

impl Definition3dPanel {
    pub fn new<'a>(
        cc: &'a eframe::CreationContext<'a>,
        camera: Option<OrbitCamera>,
        selector: &mut DefinitionSingleSelect,
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
            tracker_id: selector.register_tracker(),
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

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        frame: &eframe::Frame,
        selector: &mut DefinitionSingleSelect,
    ) {
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

        let (data, meshes) = if let Some(definition) = selector.selected() {
            definition.borrow_mut().load_data_meshes()
        } else {
            (None, None)
        };

        let mesh_loaded = meshes.is_some();
        let mesh_loaded_now = mesh_loaded != self.mesh_loaded;
        self.mesh_loaded = mesh_loaded;

        let scene_state_changed = self.scene.state_ui(ui, &meshes);
        if mesh_loaded_now
            || selector.check_update(self.tracker_id).unwrap_or(false)
            || scene_state_changed
        {
            self.scene.update(&data, &meshes);
        }

        ui.separator();

        if ui.button("Save image").clicked() {
            self.save_image_modal.open(selector.selected());
        }

        self.save_image_modal.ui(ui, frame, selector.selected());
    }
}
