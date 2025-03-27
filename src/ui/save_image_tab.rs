use super::{
    paint_canvas_3d, utils, AutoCamera, BlockViewScene, DefinitionPointer, DefinitionSelect,
    DefinitionSelectPanel, DefinitionSingleSelect, DefinitionsStore, ImageRenderer,
    SaveImageProgress, State, Tab,
};
use crate::gl_renderer::{MultisampleFramebuffer, SceneRenderer};
use eframe::glow::Context;
use egui::{
    Align, CentralPanel, DragValue, Frame, Grid, Id, Layout, Modal, ProgressBar, SidePanel, Sides,
    Slider,
};
use egui_extras::{Size, StripBuilder};
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SaveImageTab {
    width: i32,
    height: i32,
    auto_camera: AutoCamera,

    #[serde(skip)]
    definition_select_panel: DefinitionSelectPanel,
    #[serde(skip)]
    selector_observer_id: u32,

    #[serde(skip)]
    gl: Option<Arc<Context>>,
    #[serde(skip)]
    scene: BlockViewScene,
    #[serde(skip)]
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    #[serde(skip)]
    mesh_loaded: bool,

    #[serde(skip)]
    save_progress: Option<SaveImageProgress>,
    #[serde(skip)]
    framebuffer_render: Option<ImageRenderer>,
}

impl Default for SaveImageTab {
    fn default() -> Self {
        let width = 512;
        let height = 512;
        let auto_camera = AutoCamera::default();

        let mut definition_select_panel = DefinitionSelectPanel::multi_select();
        let selector_observer_id = definition_select_panel.register_observer();

        Self {
            width,
            height,
            auto_camera,
            definition_select_panel,
            selector_observer_id,
            gl: None,
            scene: Default::default(),
            renderer: None,
            mesh_loaded: false,
            save_progress: None,
            framebuffer_render: None,
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
                        let canvas_size =
                            utils::fit_size_aspect(ui.available_size(), self.aspect_ratio());

                        utils::ui_center(ui, canvas_size, |ui| {
                            egui::Frame::canvas(ui.style())
                                .inner_margin(0.0)
                                .show(ui, |ui| {
                                    let (rect, response) = ui.allocate_exact_size(
                                        canvas_size - egui::vec2(2.0, 2.0),
                                        egui::Sense::drag(),
                                    );
                                    self.auto_camera.control(ui, response);

                                    super::paint_checker_pattern(ui, rect);

                                    if self.save_progress.is_none() {
                                        if let Some(renderer) = &self.renderer {
                                            paint_canvas_3d(
                                                ui,
                                                rect,
                                                self.auto_camera.camera.clone(),
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

        if let Some(renderer) = &mut self.framebuffer_render {
            renderer.update();
        }

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

        if let Some(data) = definition
            .as_ref()
            .and_then(|d| d.lock().ok())
            .and_then(|mut d| d.load_data())
            .and_then(|d| d.ok())
        {
            self.auto_camera.update(&data);
        }

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
                    .selectable_label(!self.auto_camera.is_orthographic, "Perspective")
                    .clicked()
                {
                    self.auto_camera.is_orthographic = false;
                }
                if ui
                    .selectable_label(self.auto_camera.is_orthographic, "Orthographic")
                    .clicked()
                {
                    self.auto_camera.is_orthographic = true;
                }
            });
            ui.end_row();

            if !self.auto_camera.is_orthographic {
                let mut fov_deg = self.auto_camera.fov_y.to_degrees();
                ui.label("Field of view");
                ui.add(Slider::new(&mut fov_deg, 5.0..=150.0).suffix("°"));
                ui.end_row();
                self.auto_camera.fov_y = fov_deg.to_radians();
            }

            ui.label("Camera position");
            ui.checkbox(&mut self.auto_camera.is_auto, "Auto");
            ui.end_row();

            if self.auto_camera.is_auto {
                ui.label("Margin");
                ui.add(
                    Slider::new(
                        &mut self.auto_camera.margin,
                        0.0..=((self.width.min(self.height) - 10) / 2) as f32,
                    )
                    .suffix("px"),
                );
                ui.end_row();
            }

            let camera = &mut self.auto_camera.camera;
            let mut direction_changed = false;

            let mut distance = camera.direction.length();
            if !self.auto_camera.is_auto {
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
            Modal::new(Id::new("save_image_progress_modal"))
                .frame(Frame::popup(&ctx.style()).inner_margin(20.0))
                .show(ctx, |ui| {
                    ui.set_width(400.0);
                    ui.style_mut().spacing.item_spacing.y = 16.0;

                    ui.heading("Save images");
                    ui.add_space(10.0);
                    ui.add(ProgressBar::new(progress.progress()).show_percentage());
                    ui.add_space(10.0);
                    Sides::new().show(
                        ui,
                        |ui| {
                            if let Some(message) = &progress.message() {
                                ui.label(message);
                            }
                        },
                        |ui| {
                            ui.label(format!("{} / {}", progress.current(), progress.total()));
                        },
                    )
                });
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_image<W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        &mut self,
        definitions: Vec<DefinitionPointer>,
        dialog_parent: Option<&W>,
    ) {
        use crate::ui::{ImageRenderer, ProgressMessage};

        use super::file_dialog;
        use std::{cmp::Ordering, sync::mpsc, thread};

        if let Some(gl) = &self.gl {
            // 保存場所のダイアログ
            let (save_path, is_single) = match definitions.len().cmp(&1) {
                Ordering::Greater => (file_dialog::open_folder_dialog(dialog_parent), false),
                Ordering::Equal => {
                    let filename = definitions[0]
                        .lock()
                        .ok()
                        .map(|d| utils::replace_extension(&d.filename(), "png"));
                    (
                        file_dialog::save_png_dialog(dialog_parent, filename.as_deref()),
                        true,
                    )
                }
                Ordering::Less => {
                    return;
                }
            };
            if save_path.is_none() {
                return;
            }
            let save_path = save_path.unwrap();

            let (tx_progress, rx_progress) = mpsc::channel();
            let (tx_render, rx_render) = mpsc::channel();
            let progress = SaveImageProgress::new(rx_progress, definitions.len());
            self.save_progress = Some(progress);

            let scene = BlockViewScene::clone_state(&self.scene);
            let renderer = SceneRenderer::new(gl, scene.scene());
            self.framebuffer_render = Some(ImageRenderer::new(
                rx_render,
                scene,
                renderer,
                &self.auto_camera,
                MultisampleFramebuffer::new(gl.clone(), self.width, self.height, 8),
                save_path,
                !is_single,
            ));

            // 読み込みは別スレッドで行うが、描画はメインスレッドで行う
            thread::spawn(move || {
                let start_time = std::time::Instant::now();

                for (i, definition) in definitions.iter().enumerate() {
                    let _ = tx_progress.send(ProgressMessage::Progress(i));

                    if let Ok(mut definition) = definition.lock() {
                        if let Some((data, meshes)) = definition
                            .load_data_block()
                            .ok()
                            .and_then(|d| d.ok())
                            .zip(definition.load_meshes_block().ok())
                        {
                            let _ = tx_render.send((data, meshes, definition.filename()));
                        }
                    }
                }

                if start_time.elapsed().as_millis() >= 1000 {
                    // 1秒以上かかったら少しの間100%と表示する
                    let _ = tx_progress.send(ProgressMessage::Progress(definitions.len()));
                    thread::sleep(std::time::Duration::from_secs(2));
                }
                let _ = tx_progress.send(ProgressMessage::Done);
            });
        }
    }
}
