use super::{paint_canvas_3d, BlockViewScene};
use crate::{
    gl_renderer::{Camera, OrbitCamera, SceneRenderer},
    sw_block_definition::SwBlockDefinition,
};
use eframe::glow::Context;
use egui::{vec2, Align, DragValue, Grid, Id, Layout, Modal, Sides, Slider};
use egui_extras::{Size, StripBuilder};
use glam::{Vec3, Vec4};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveImageModal {
    #[serde(skip)]
    open: bool,
    width: i32,
    height: i32,
    is_orthographic: bool,
    fov: f32,
    camera_auto: bool,
    margin: i32,
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
            camera_auto: false,
            margin: 0,
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

    pub fn open(&mut self, definition: Option<Rc<RefCell<SwBlockDefinition>>>) {
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

    pub fn update_scene(&mut self, definition: Option<Rc<RefCell<SwBlockDefinition>>>) {
        if let Some(definition) = definition {
            let (data, meshes) = definition.borrow_mut().load_data_meshes();
            self.scene.update(&data, &meshes);
        }
    }

    #[allow(unused_variables)]
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        frame: &eframe::Frame,
        definition: Option<Rc<RefCell<SwBlockDefinition>>>,
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

        let definition_c = definition.clone();

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

            if self.camera_auto {
                if let Some(data) = definition
                    .as_ref()
                    .and_then(|d| d.borrow_mut().load_data())
                    .and_then(|d| d.ok())
                {
                    let voxel_min: Option<Vec3> = data.voxel_min.last().map(|v| (*v).into());
                    let voxel_max: Option<Vec3> = data.voxel_max.last().map(|v| (*v).into());
                    let corner_min: Vec3 = (voxel_min.unwrap_or_default() - 0.5 * Vec3::ONE) * 0.25;
                    let corner_max: Vec3 = (voxel_max.unwrap_or_default() + 0.5 * Vec3::ONE) * 0.25;
                    let center = (corner_min + corner_max) * 0.5;

                    let min_x = corner_min.x;
                    let min_y = corner_min.y;
                    let min_z = corner_min.z;
                    let max_x = corner_max.x;
                    let max_y = corner_max.y;
                    let max_z = corner_max.z;
                    let corners = [
                        Vec3::new(min_x, min_y, min_z),
                        Vec3::new(min_x, max_y, min_z),
                        Vec3::new(min_x, max_y, max_z),
                        Vec3::new(min_x, min_y, max_z),
                        Vec3::new(max_x, min_y, min_z),
                        Vec3::new(max_x, max_y, min_z),
                        Vec3::new(max_x, max_y, max_z),
                        Vec3::new(max_x, min_y, max_z),
                    ];

                    camera.center = Vec3::new(center.x, center.y, -center.z);

                    let s = if self.is_orthographic {
                        // 平行投影
                        let mat_vp = camera.mat_view_proj();
                        let (screen_min_x, screen_min_y, screen_max_x, screen_max_y) =
                            corners.iter().fold(
                                (
                                    f32::INFINITY,
                                    f32::INFINITY,
                                    f32::NEG_INFINITY,
                                    f32::NEG_INFINITY,
                                ),
                                |(min_x, min_y, max_x, max_y), c| {
                                    let s = mat_vp.mul_vec4(Vec4::new(c.x, c.y, -c.z, 1.0));
                                    (
                                        min_x.min(s.x),
                                        min_y.min(s.y),
                                        max_x.max(s.x),
                                        max_y.max(s.y),
                                    )
                                },
                            );

                        let sx = (-screen_min_x).max(screen_max_x)
                            / ((self.width - 2 * self.margin) as f32 / self.width as f32);
                        let sy = (-screen_min_y).max(screen_max_y)
                            / ((self.height - 2 * self.margin) as f32 / self.height as f32);
                        sx.max(sy)
                    } else {
                        // 透視投影
                        // 中心をバウンディングボックスの中心にしているが、角度により片側に偏って見えてしまうので、できれば直す
                        let view = camera.mat_view();
                        let tan = (self.fov.to_radians() / 2.0).tan();
                        let tan_x = (self.width - 2 * self.margin) as f32 / self.width as f32
                            * tan
                            * aspect_ratio;
                        let tan_y =
                            (self.height - 2 * self.margin) as f32 / self.height as f32 * tan;
                        let len = camera.direction.length();
                        corners.iter().fold(0.0, |s: f32, corner| {
                            let view_point =
                                view.transform_point3(Vec3::new(corner.x, corner.y, -corner.z));
                            let dx = (view_point.x.abs() / tan_x) - (-view_point.z);
                            let dy = (view_point.y.abs() / tan_y) - (-view_point.z);
                            let sx = (len + dx) / len;
                            let sy = (len + dy) / len;
                            s.max(sx.max(sy))
                        })
                    };
                    camera.direction *= s;
                }
            }
        }
        if let Some(definition) = definition {
            let (data, meshes) = definition.borrow_mut().load_data_meshes();
            self.scene
                .set_orthographic(self.is_orthographic, &data, &meshes);
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

            ui.label("Camera position");
            ui.checkbox(&mut self.camera_auto, "Auto");
            ui.end_row();

            if self.camera_auto {
                ui.label("Margin");
                ui.add(
                    Slider::new(
                        &mut self.margin,
                        0..=((self.width.min(self.height) - 10) / 2),
                    )
                    .suffix("px"),
                );
                ui.end_row();
            }

            if let Ok(mut camera) = self.camera.lock() {
                let mut direction_changed = false;

                let mut distance = camera.direction.length();
                if !self.camera_auto {
                    ui.label("Look at");
                    let mut z = -camera.center.z;
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut camera.center.x).speed(0.01));
                        ui.add(DragValue::new(&mut camera.center.y).speed(0.01));
                        ui.add(DragValue::new(&mut z).speed(0.01));
                    });
                    camera.center.z = -z;
                    ui.end_row();

                    ui.label("Distance");
                    direction_changed |= ui
                        .add(
                            Slider::new(&mut distance, 0.1..=100.0)
                                .logarithmic(true)
                                .clamping(egui::SliderClamping::Never),
                        )
                        .changed();
                    ui.end_row();
                }

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

                if direction_changed {
                    camera.set_direction_angle(
                        azimuth_angle.to_radians(),
                        elevation_angle.to_radians(),
                        distance,
                    );
                }
            }
        });
    }

    fn ui_scene(&mut self, ui: &mut egui::Ui, definition: Option<Rc<RefCell<SwBlockDefinition>>>) {
        let (data, meshes) = if let Some(definition) = definition {
            definition.borrow_mut().load_data_meshes()
        } else {
            (None, None)
        };

        let mesh_loaded = meshes.is_some();
        let mesh_loaded_now = mesh_loaded != self.mesh_loaded;
        self.mesh_loaded = mesh_loaded;

        let scene_state_changed = self.scene.state_ui(ui, &meshes);
        if mesh_loaded_now || scene_state_changed {
            self.scene.update(&data, &meshes);
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
