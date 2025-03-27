use super::{definitions_store::DefinitionPointer, paint_canvas_3d, BlockViewScene};
use crate::gl_renderer::{OrbitCamera, SceneRenderer};
use egui::vec2;
use glam::Vec3;
use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Definition3dPanel {
    #[serde(skip)]
    scene: BlockViewScene,
    camera: Arc<Mutex<OrbitCamera>>,
    #[serde(skip)]
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    #[serde(skip)]
    mesh_loaded: bool,
}

impl Definition3dPanel {
    pub fn new(camera: Option<OrbitCamera>) -> Self {
        let camera = camera.unwrap_or_else(|| {
            OrbitCamera::new(
                Vec3::ZERO,
                Vec3::new(1.0, -0.5, 1.0),
                45f32.to_radians(),
                1.0,
            )
        });

        Self {
            scene: Default::default(),
            camera: Arc::new(Mutex::new(camera)),
            renderer: None,
            mesh_loaded: false,
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

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        selected: Option<DefinitionPointer>,
        select_changed: bool,
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

        let mut data = None;
        let mut meshes = None;
        if let Some(definition) = selected {
            if let Ok(mut definition) = definition.lock() {
                (data, meshes) = definition.load_data_meshes()
            }
        }

        let mesh_loaded = meshes.is_some();
        let mesh_loaded_now = mesh_loaded != self.mesh_loaded;
        self.mesh_loaded = mesh_loaded;

        let scene_state_changed = self.scene.state_ui(ui, &meshes);
        if mesh_loaded_now || select_changed || scene_state_changed {
            self.scene.update(&data, &meshes);
        }
    }
}
