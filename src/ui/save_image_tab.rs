use super::{
    definitions_store::DefinitionPointer, paint_canvas_3d, tab::Tab, BlockViewScene,
    DefinitionSelect, DefinitionSelectPanel, DefinitionSingleSelect, DefinitionsStore, State,
};
use crate::gl_renderer::{Camera, MultisampleFramebuffer, OrbitCamera, SceneRenderer};
use eframe::glow::Context;
use egui::{
    Align, CentralPanel, DragValue, Grid, Id, Layout, Modal, ProgressBar, Rect, SidePanel, Sides,
    Slider, UiBuilder,
};
use egui_extras::{Size, StripBuilder};
use glam::{Vec3, Vec4};
use std::sync::{mpsc, Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SaveImageTab {
    width: i32,
    height: i32,
    is_orthographic: bool,
    fov: f32,
    camera_auto: bool,
    margin: i32,

    #[serde(skip)]
    definition_select_panel: DefinitionSelectPanel,
    #[serde(skip)]
    selector_observer_id: u32,

    #[serde(skip)]
    gl: Option<Arc<Context>>,
    #[serde(skip)]
    scene: BlockViewScene,
    camera: Arc<Mutex<OrbitCamera>>,
    #[serde(skip)]
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    #[serde(skip)]
    mesh_loaded: bool,

    #[serde(skip)]
    save_progress: Option<SaveImageProgress>,
}

impl Default for SaveImageTab {
    fn default() -> Self {
        let width = 512;
        let height = 512;
        let fov: f32 = 45.0;

        let camera = Arc::new(Mutex::new(OrbitCamera::new(
            Vec3::ZERO,
            Vec3::new(1.0, -0.5, 1.0),
            fov.to_radians(),
            width as f32 / height as f32,
        )));

        let mut definition_select_panel = DefinitionSelectPanel::multi_select();
        let selector_observer_id = definition_select_panel.register_observer();

        Self {
            width,
            height,
            is_orthographic: false,
            fov,
            camera_auto: false,
            margin: 0,
            definition_select_panel,
            selector_observer_id,
            gl: None,
            scene: Default::default(),
            camera,
            renderer: None,
            mesh_loaded: false,
            save_progress: None,
        }
    }
}

impl Tab for SaveImageTab {
    fn creation_context<'a>(&mut self, cc: &'a eframe::CreationContext<'a>) {
        if let Some(gl) = &cc.gl {
            let renderer = SceneRenderer::new(gl, self.scene.scene());
            self.renderer = Some(Arc::new(egui::mutex::Mutex::new(renderer)));
            self.gl = Some(gl.clone());
        }
    }

    fn use_selector(&mut self, selector: std::rc::Rc<std::cell::RefCell<DefinitionSingleSelect>>) {
        self.selector_observer_id = selector.borrow_mut().register_observer();
        self.definition_select_panel.use_selector(selector);
    }

    fn destroy(&mut self, gl: Option<&eframe::glow::Context>) {
        if let Some(renderer) = &self.renderer {
            renderer.lock().destroy(gl);
        }
    }

    #[allow(unused_variables)]
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
        _state: &mut State,
        definitions_store: &mut DefinitionsStore,
    ) {
        SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(80.0..=500.0)
            .show(ctx, |ui| {
                self.definition_select_panel.ui(ui, definitions_store);
            });

        let definition = self.definition_select_panel.selected_definition();

        CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::initial(240.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        let canvas_size = fit_size_aspect(ui.available_size(), self.aspect_ratio());

                        ui_center(ui, canvas_size, |ui| {
                            egui::Frame::canvas(ui.style())
                                .inner_margin(0.0)
                                .show(ui, |ui| {
                                    let (rect, response) = ui.allocate_exact_size(
                                        canvas_size - egui::vec2(2.0, 2.0),
                                        egui::Sense::drag(),
                                    );
                                    self.camera.lock().unwrap().control(ui, response);

                                    super::paint_checker_pattern(ui, rect);

                                    if self.save_progress.is_none() {
                                        if let Some(renderer) = &self.renderer {
                                            paint_canvas_3d(
                                                ui,
                                                rect,
                                                self.camera.clone(),
                                                renderer.clone(),
                                            );
                                        }
                                    }
                                });
                        });
                    });

                    strip.strip(|strip| {
                        strip
                            .sizes(Size::remainder(), 2)
                            .size(Size::initial(120.0))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    self.ui_camera_params(ui, Id::new("save_image_camera_params"));
                                });
                                strip.cell(|ui| {
                                    self.ui_scene(ui, &definition);
                                });
                                strip.cell(|ui| {
                                    self.ui_buttons(ui, frame);
                                });
                            });
                    });
                });
        });

        if let Some(progress) = &mut self.save_progress {
            progress.update();
            if progress.done() {
                self.save_progress = None;
            } else {
                self.ui_progress_modal(ctx);
                ctx.request_repaint();
                return;
            }
        }

        self.camera_control(&definition);

        if self
            .definition_select_panel
            .check_update(self.selector_observer_id)
            .unwrap_or(false)
        {
            self.update_scene(&definition);
        }
    }
}

impl SaveImageTab {
    fn update_scene(&mut self, definition: &Option<DefinitionPointer>) {
        if let Some(definition) = definition {
            let (data, meshes) = definition
                .lock()
                .ok()
                .map(|mut d| d.load_data_meshes())
                .unwrap_or((None, None));
            self.scene.update(&data, &meshes);
        } else {
            self.scene.clear();
        }
    }

    fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    fn camera_control(&self, definition: &Option<DefinitionPointer>) {
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
                    .and_then(|d| d.lock().ok())
                    .and_then(|mut d| d.load_data())
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
                            * self.aspect_ratio();
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

    fn ui_scene(&mut self, ui: &mut egui::Ui, definition: &Option<DefinitionPointer>) {
        let (data, meshes) = definition
            .as_ref()
            .and_then(|d| d.lock().ok())
            .map(|mut d| d.load_data_meshes())
            .unwrap_or((None, None));

        let mesh_loaded = meshes.is_some();
        let mesh_loaded_now = mesh_loaded != self.mesh_loaded;
        self.mesh_loaded = mesh_loaded;

        let scene_state_changed = self.scene.state_ui(ui, &meshes);
        if mesh_loaded_now || scene_state_changed {
            self.scene.update(&data, &meshes);
        }
    }

    fn ui_buttons(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(multi_selector) = self.definition_select_panel.multi_selector() {
                let count = multi_selector.borrow().count();
                let mut save_definitions = None;

                if count > 0 {
                    let button = ui.button(format!("Save {} images", count));
                    if button.clicked() {
                        save_definitions = Some(multi_selector.borrow().selection());
                    }
                } else if let Some(definition) = self.definition_select_panel.selected_definition()
                {
                    let button = ui.button("Save image");
                    if button.clicked() {
                        save_definitions = Some(vec![definition]);
                    }
                }

                if let Some(definitions) = save_definitions {
                    self.save_image(definitions, Some(frame));
                }
            }
        });
    }

    fn ui_progress_modal(&self, ctx: &eframe::egui::Context) {
        if let Some(progress) = &self.save_progress {
            Modal::new(Id::new("save_image_progress_modal")).show(ctx, |ui| {
                ui.set_width(400.0);

                ui.heading("Save images");
                ui.add_space(10.0);
                ui.add(ProgressBar::new(progress.progress()).show_percentage());
                ui.add_space(10.0);
                Sides::new().show(
                    ui,
                    |ui| {
                        if let Some(message) = &progress.message {
                            ui.label(message);
                        }
                    },
                    |ui| {
                        ui.label(format!("{} / {}", progress.current, progress.total));
                    },
                )
            });
        }
    }

    /*fn save_images(&mut self, definitions: &[DefinitionPointer]) {
        self.save_progress = Some(0.0);
        let total_count = definitions.len();

        for (i, definition) in definitions.iter().enumerate() {
            self.update_scene(&Some(definition.clone()));
            self.save_progress = Some(i as f32 / total_count as f32);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_current_image<
        W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
    >(
        &self,
        parent: Option<&W>,
        initial_filename: Option<&str>,
    ) {
        use crate::{gl_renderer::MultisampleFramebuffer, ui::file_dialog};

        if let Some((gl, renderer)) = self.gl.as_ref().zip(self.renderer.as_ref()) {
            let framebuffer = MultisampleFramebuffer::new(gl.clone(), self.width, self.height, 8);

            framebuffer.bind();
            renderer
                .lock()
                .paint(&framebuffer.gl(), self.camera.clone());
            framebuffer.resolve();
            let image = framebuffer.get_image();

            if let Some(path) = file_dialog::save_image_dialog(parent, initial_filename) {
                image.save(path).expect("Failed to save image");
            }
        }
    }*/

    #[cfg(not(target_arch = "wasm32"))]
    fn save_image<W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        &mut self,
        definitions: Vec<DefinitionPointer>,
        dialog_parent: Option<&W>,
    ) {
        use super::file_dialog;
        use std::{cmp::Ordering, sync::mpsc, thread};

        if let Some(_gl) = &self.gl {
            // 保存場所のダイアログ
            let _save_path = match definitions.len().cmp(&1) {
                Ordering::Greater => file_dialog::open_folder_dialog(dialog_parent),
                Ordering::Equal => {
                    let filename = definitions[0]
                        .lock()
                        .ok()
                        .map(|d| replace_extension(&d.filename(), "png"));
                    file_dialog::save_png_dialog(dialog_parent, filename.as_deref())
                }
                Ordering::Less => {
                    return;
                }
            };

            let (tx, rx) = mpsc::channel();
            let progress = SaveImageProgress::new(rx, definitions.len());
            self.save_progress = Some(progress);

            /*let gl_c = gl.clone();
            let width = self.width;
            let height = self.height;*/

            thread::spawn(move || {
                /*let scene = BlockViewScene::default();
                let _renderer = SceneRenderer::new(&gl_c, scene.scene());
                let _framebuffer = MultisampleFramebuffer::new(gl_c, width, height, 8);*/

                for (i, _definition) in definitions.iter().enumerate() {
                    let _ = tx.send(ProgressMessage::Progress(i));
                    println!("send {}", i);

                    //let (data, meshes) = definition.borrow_mut().load_data_meshes();

                    // ダミーの待ち時間
                    thread::sleep(std::time::Duration::from_secs(1));
                }
                let _ = tx.send(ProgressMessage::Done);
            });
        }
    }
}

fn fit_size_aspect(size: egui::Vec2, aspect_ratio: f32) -> egui::Vec2 {
    let width = size.x;
    let height = size.y;
    if width / height > aspect_ratio {
        egui::vec2(height * aspect_ratio, height)
    } else {
        egui::vec2(width, width / aspect_ratio)
    }
}

fn ui_center(ui: &mut egui::Ui, size: egui::Vec2, add_contents: impl FnOnce(&mut egui::Ui)) {
    let rect = Rect::from_center_size(ui.available_rect_before_wrap().center(), size);
    ui.allocate_new_ui(UiBuilder::new().max_rect(rect), add_contents);
}

fn replace_extension(filename: &str, new_ext: &str) -> String {
    let mut path = std::path::Path::new(filename).to_owned();
    path.set_extension(new_ext);
    path.to_string_lossy().into_owned()
}

fn _paint_on_framebuffer(
    framebuffer: MultisampleFramebuffer,
    renderer: &mut SceneRenderer,
    camera: Arc<Mutex<impl Camera>>,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    framebuffer.bind();
    renderer.paint(&framebuffer.gl(), camera);
    framebuffer.resolve();
    framebuffer.get_image()
}

#[derive(Debug)]
enum ProgressMessage {
    Progress(usize),
    Done,
}

#[derive(Debug)]
struct SaveImageProgress {
    rx: mpsc::Receiver<ProgressMessage>,
    current: usize,
    total: usize,
    done: bool,
    message: Option<String>,
}

impl SaveImageProgress {
    fn new(rx: mpsc::Receiver<ProgressMessage>, total: usize) -> Self {
        Self {
            rx,
            current: 0,
            total,
            done: false,
            message: None,
        }
    }

    fn update(&mut self) {
        if let Ok(mes) = self.rx.try_recv() {
            match mes {
                ProgressMessage::Progress(value) => {
                    self.current = value;
                }
                ProgressMessage::Done => {
                    self.done = true;
                }
            }
        }
    }

    fn progress(&self) -> f32 {
        self.current as f32 / self.total as f32
    }

    fn done(&self) -> bool {
        self.done
    }
}
