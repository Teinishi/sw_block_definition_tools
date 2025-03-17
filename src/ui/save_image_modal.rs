use super::{paint_canvas_3d, BlockViewScene};
use crate::{
    gl_renderer::{OrbitCamera, SceneRenderer},
    sw_block_definition::SwBlockDefinition,
};
use eframe::glow::Context;
use egui::{vec2, Align, DragValue, Grid, Id, Layout, Modal, Slider};
use glam::Vec3;
use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveImageModal {
    #[serde(skip)]
    open: bool,
    width: i32,
    height: i32,
    is_orthographic: bool,
    fov: f32,
    zoom: f32,
    #[serde(skip)]
    gl: Option<Arc<Context>>,
    #[serde(skip)]
    scene: BlockViewScene,
    camera: Arc<Mutex<OrbitCamera>>,
    #[serde(skip)]
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    #[serde(skip)]
    mesh_loaded: bool,
}

impl Default for SaveImageModal {
    fn default() -> Self {
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
            is_orthographic: false,
            fov: 60.0,
            zoom: 1.0,
            gl: None,
            scene: Default::default(),
            camera,
            renderer: None,
            mesh_loaded: false,
        }
    }
}

impl SaveImageModal {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Self {
        let mut instance = Self::default();
        instance.creation_context(cc);
        instance
    }

    pub fn open(&mut self, definition: Option<&mut SwBlockDefinition>) {
        self.open = true;
        if let Some(definition) = definition {
            let data = definition.load_data();
            let meshes = definition.load_meshes();
            self.scene.update(data.and_then(|d| d.ok()), meshes);
        }
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn creation_context<'a>(&mut self, cc: &'a eframe::CreationContext<'a>) {
        if let Some(gl) = &cc.gl {
            let renderer = SceneRenderer::new(gl, self.scene.scene());
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
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        frame: &eframe::Frame,
        definition: Option<&mut SwBlockDefinition>,
    ) {
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
                let max_height = 0.4 * viewport_size.y;
                let width = max_width.min(max_height * aspect_ratio);
                vec2(width, width / aspect_ratio)
            });
            if let Ok(mut camera) = self.camera.lock() {
                camera.set_aspect_ratio(aspect_ratio);
                if self.is_orthographic {
                    camera.set_orthographic();
                } else {
                    camera.set_perspective();
                    camera.set_fov_y(self.fov.to_radians());
                }
            }
            let container_size = viewport_size
                .map(|viewport_size| vec2(0.7 * viewport_size.x, 0.4 * viewport_size.y))
                .unwrap();

            ui.allocate_ui_with_layout(container_size, Layout::top_down(Align::Center), |ui| {
                egui::Frame::new().show(ui, |ui| {
                    let (rect, response) = ui.allocate_exact_size(
                        fit_size_aspect(container_size, aspect_ratio),
                        egui::Sense::drag(),
                    );
                    self.camera.lock().unwrap().control(ui, response);

                    super::paint_checker_pattern(ui, rect);
                    if let Some(renderer) = &self.renderer {
                        paint_canvas_3d(ui, rect, self.camera.clone(), renderer.clone());
                    }
                });
            });

            ui.add_space(8.0);

            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                Grid::new(id.with("params"))
                    .spacing([10.0, 6.0])
                    .show(ui, |ui| {
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

                        ui.label("Camera type");
                        ui.horizontal(|ui| {
                            if ui
                                .selectable_label(!self.is_orthographic, "Perspective")
                                .clicked()
                            {
                                self.is_orthographic = false;
                            }
                            if ui
                                .selectable_label(self.is_orthographic, "Orthographic")
                                .clicked()
                            {
                                self.is_orthographic = true;
                            }
                        });
                        ui.end_row();

                        if !self.is_orthographic {
                            ui.label("Field of view");
                            ui.add(Slider::new(&mut self.fov, 5.0..=150.0).suffix("°"));
                            ui.end_row();
                        }
                    });

                ui.add_space(8.0);

                let (data, meshes) = if let Some(definition) = definition {
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

                ui.add_space(8.0);

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

fn fit_size_aspect(size: egui::Vec2, aspect_ratio: f32) -> egui::Vec2 {
    let width = size.x;
    let height = size.y;
    if width / height > aspect_ratio {
        vec2(height * aspect_ratio, height)
    } else {
        vec2(width, width / aspect_ratio)
    }
}
