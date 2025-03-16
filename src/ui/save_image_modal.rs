use super::paint_canvas_3d;
use crate::gl_renderer::{OrbitCamera, Scene, SceneRenderer};
use eframe::glow::Context;
use egui::{vec2, DragValue, Grid, Id, Modal, Slider};
use glam::Vec3;
use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveImageModal {
    #[serde(skip)]
    open: bool,
    width: i32,
    height: i32,
    fov: f32,
    #[serde(skip)]
    gl: Option<Arc<Context>>,
    #[serde(skip)]
    scene: Arc<Mutex<Scene>>,
    camera: Arc<Mutex<OrbitCamera>>,
    #[serde(skip)]
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
}

impl Default for SaveImageModal {
    fn default() -> Self {
        let scene = Arc::new(Mutex::new(Scene::default()));
        let mut camera = OrbitCamera {
            direction: Vec3::new(1.0, -0.5, -1.0),
            ..Default::default()
        };
        camera.orthogonalize_up();
        let camera = Arc::new(Mutex::new(camera));

        Self {
            open: false,
            width: 512,
            height: 512,
            fov: 60.0,
            gl: None,
            scene: scene.clone(),
            camera,
            renderer: None,
        }
    }
}

impl SaveImageModal {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Self {
        let mut instance = Self::default();
        instance.creation_context(cc);
        instance
    }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn creation_context<'a>(&mut self, cc: &'a eframe::CreationContext<'a>) {
        if let Some(gl) = &cc.gl {
            let renderer = SceneRenderer::new(gl, self.scene.clone());
            self.renderer = Some(Arc::new(egui::mutex::Mutex::new(renderer)));
            self.gl = Some(gl.clone());
        }
    }

    pub fn destroy(&self, gl: Option<&eframe::glow::Context>) {
        if let Some(renderer) = &self.renderer {
            renderer.lock().destroy(gl);
        }
    }

    #[allow(unused_variables)]
    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &eframe::Frame) {
        if !self.open {
            return;
        }

        let id = Id::new("save_image_modal");
        let modal = Modal::new(id).show(ui.ctx(), |ui| {
            let viewport_size: Option<egui::Vec2> =
                ui.ctx().input(|i| Some(i.viewport().inner_rect?.size()));

            let aspect_ratio = self.width as f32 / self.height as f32;
            let canvas_size = viewport_size.map(|viewport_size| {
                let max_width = 0.7 * viewport_size.x;
                let max_height = 0.5 * viewport_size.y;
                let width = max_width.min(max_height * aspect_ratio);
                vec2(width, width / aspect_ratio)
            });

            egui::Frame::canvas(ui.style())
                .fill(egui::Color32::TRANSPARENT)
                .inner_margin(0.0)
                .show(ui, |ui| {
                    let (rect, response) = ui.allocate_exact_size(
                        canvas_size.unwrap_or(vec2(self.width as f32, self.height as f32)),
                        egui::Sense::drag(),
                    );
                    self.camera.lock().unwrap().control(ui, response);

                    super::paint_checker_pattern(ui, rect);
                    if let Some(renderer) = &self.renderer {
                        paint_canvas_3d(ui, rect, self.camera.clone(), renderer.clone());
                    }
                });

            ui.add_space(4.0);

            Grid::new(id.with("params")).show(ui, |ui| {
                ui.label("Size");
                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut self.width)
                            .range(1..=10000)
                            .suffix("px"),
                    );
                    ui.label("x");
                    ui.add(
                        DragValue::new(&mut self.height)
                            .range(1..=10000)
                            .suffix("px"),
                    );
                });
                ui.end_row();

                ui.label("Field of view");
                ui.add(Slider::new(&mut self.fov, 0.0..=180.0).suffix("°"));
                ui.end_row();
            });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    self.close();
                }

                if ui.button("Save image").clicked() {
                    #[cfg(not(target_arch = "wasm32"))]
                    self.save_image(Some(frame));
                }
            });
        });

        if modal.should_close() {
            self.close();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_image<W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        &self,
        parent: Option<&W>,
    ) {
        use crate::gl_renderer::MultisampleFramebuffer;

        if let Some((gl, renderer)) = self.gl.as_ref().zip(self.renderer.as_ref()) {
            let framebuffer = MultisampleFramebuffer::new(gl.clone(), self.width, self.height, 8);

            framebuffer.bind();
            renderer
                .lock()
                .paint(&framebuffer.gl(), self.camera.clone());
            framebuffer.resolve();
            let image = framebuffer.get_image();

            if let Some(path) = save_image_dialog(parent) {
                image.save(path).expect("Failed to save image");
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_image_dialog<
    W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
>(
    parent: Option<&W>,
) -> Option<std::path::PathBuf> {
    use rfd::FileDialog;

    let mut dialog = FileDialog::new().add_filter("PNG image", &["png"]);
    if let Some(p) = parent {
        dialog = dialog.set_parent(p)
    }
    dialog.save_file()
}
