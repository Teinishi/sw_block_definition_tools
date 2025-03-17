use super::{paint_canvas_3d, BlockViewScene};
use crate::{
    gl_renderer::{OrbitCamera, SceneRenderer},
    sw_block_definition::SwBlockDefinition,
};
use eframe::glow::Context;
use egui::{vec2, Align, DragValue, Grid, Id, Layout, Modal, Sides, Slider};
use egui_extras::{Size, StripBuilder};
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
        self.update_scene(definition);
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

    pub fn update_scene(&mut self, definition: Option<&mut SwBlockDefinition>) {
        if let Some(definition) = definition {
            let data = definition.load_data();
            let meshes = definition.load_meshes();
            self.scene.update(data.and_then(|d| d.ok()), meshes);
        }
    }

    #[allow(unused_variables)]
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        frame: &eframe::Frame,
        definition: &mut Option<&mut SwBlockDefinition>,
    ) {
        if !self.open {
            return;
        }

        let viewport_size: Option<egui::Vec2> =
            ui.ctx().input(|i| Some(i.viewport().inner_rect?.size()));
        let view_container_size = viewport_size
            .map(|viewport_size| vec2(0.7 * viewport_size.x, 0.4 * viewport_size.y))
            .unwrap_or_else(|| vec2(self.width as f32, self.height as f32));
        let aspect_ratio = self.width as f32 / self.height as f32;

        let definition_c = definition.as_deref_mut();

        let id = Id::new("save_image_modal");
        let modal = Modal::new(id).show(ui.ctx(), |ui| {
            StripBuilder::new(ui)
                .size(Size::exact(view_container_size.y))
                .size(Size::initial(200.0))
                .size(Size::initial(20.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.allocate_ui_with_layout(
                            view_container_size,
                            Layout::top_down(Align::Center),
                            |ui| {
                                egui::Frame::new().show(ui, |ui| {
                                    let (rect, response) = ui.allocate_exact_size(
                                        fit_size_aspect(view_container_size, aspect_ratio),
                                        egui::Sense::drag(),
                                    );
                                    self.camera.lock().unwrap().control(ui, response);

                                    super::paint_checker_pattern(ui, rect);
                                    if let Some(renderer) = &self.renderer {
                                        paint_canvas_3d(
                                            ui,
                                            rect,
                                            self.camera.clone(),
                                            renderer.clone(),
                                        );
                                    }
                                });
                            },
                        );
                    });

                    strip.strip(|strip| {
                        strip.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                self.ui_camera_params(ui, id.with("camera_params"));
                            });
                            strip.cell(|ui| {
                                self.ui_scene(ui, definition_c);
                            });
                        });
                    });

                    strip.cell(|ui| {
                        Sides::new().show(
                            ui,
                            |_ui| {},
                            |ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("Save image").clicked() {
                                        #[cfg(not(target_arch = "wasm32"))]
                                        self.save_image(Some(frame));
                                    }
                                    if ui.button("Cancel").clicked() {
                                        self.close();
                                    }
                                });
                            },
                        );
                    });
                });
        });

        if let Ok(mut camera) = self.camera.lock() {
            camera.set_aspect_ratio(self.width as f32 / self.height as f32);
            if self.is_orthographic {
                camera.set_orthographic();
            } else {
                camera.set_perspective();
                camera.set_fov_y(self.fov.to_radians());
            }
        }
        if let Some(definition) = definition {
            let data = definition.load_data();
            let meshes = definition.load_meshes();
            self.scene
                .set_orthographic(self.is_orthographic, data.and_then(|d| d.ok()), meshes);
        }

        if modal.should_close() {
            self.close();
        }
    }

    fn ui_camera_params(&mut self, ui: &mut egui::Ui, id: Id) {
        Grid::new(id).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("Image size");
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

            if let Ok(mut camera) = self.camera.lock() {
                let mut direction_changed = false;

                let mut azimuth_angle = camera.azimuth_angle().to_degrees();
                ui.label("Azimuth angle");
                direction_changed |= ui
                    .add(
                        Slider::new(&mut azimuth_angle, -180.0..=180.0)
                            .suffix("°")
                            .drag_value_speed(0.1),
                    )
                    .changed();
                ui.end_row();

                let mut elevation_angle = camera.elevation_angle().to_degrees();
                ui.label("Elevation angle");
                direction_changed |= ui
                    .add(
                        Slider::new(&mut elevation_angle, -90.0..=90.0)
                            .suffix("°")
                            .drag_value_speed(0.1),
                    )
                    .changed();
                ui.end_row();

                let mut distance = camera.direction.length();
                ui.label("Distance");
                direction_changed |= ui
                    .add(
                        Slider::new(&mut distance, 0.1..=100.0)
                            .logarithmic(true)
                            .clamping(egui::SliderClamping::Never),
                    )
                    .changed();
                ui.end_row();

                if direction_changed {
                    camera.set_direction_angle(
                        azimuth_angle.to_radians(),
                        elevation_angle.to_radians(),
                        distance,
                    );
                }

                ui.label("Look at");
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut camera.center.x).speed(0.01));
                    ui.add(DragValue::new(&mut camera.center.y).speed(0.01));
                    ui.add(DragValue::new(&mut camera.center.z).speed(0.01));
                });
                ui.end_row();
            }
        });
    }

    fn ui_scene(&mut self, ui: &mut egui::Ui, definition: Option<&mut SwBlockDefinition>) {
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
