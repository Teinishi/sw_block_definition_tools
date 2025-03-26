use super::{paint_canvas_3d, BlockViewScene};
use crate::{
    gl_renderer::{OrbitCamera, SceneRenderer},
    sw_block_definition::SwBlockDefinition,
};
use egui::vec2;
use glam::Vec3;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

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
        let mut camera = camera.unwrap_or_else(|| OrbitCamera {
            direction: Vec3::new(1.0, -0.5, -1.0),
            ..Default::default()
        });
        camera.orthogonalize_up();
        let camera = Arc::new(Mutex::new(camera));

        Self {
            scene: Default::default(),
            camera,
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
        selected: Option<Rc<RefCell<SwBlockDefinition>>>,
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

        let (data, meshes) = if let Some(definition) = selected {
            definition.borrow_mut().load_data_meshes()
        } else {
            (None, None)
        };

        let mesh_loaded = meshes.is_some();
        let mesh_loaded_now = mesh_loaded != self.mesh_loaded;
        self.mesh_loaded = mesh_loaded;

        let scene_state_changed = self.scene.state_ui(ui, &meshes);
        if mesh_loaded_now || select_changed || scene_state_changed {
            self.scene.update(&data, &meshes);
        }
    }
}
