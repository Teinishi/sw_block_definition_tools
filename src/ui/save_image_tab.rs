use super::{
    paint_canvas_3d, utils, AutoCamera, BlockViewScene, BlockViewStateMeshOptions,
    DefinitionPointer, DefinitionSelect, DefinitionSelectPanel, DefinitionSingleSelect,
    DefinitionsStore, ImageRenderer, State, Tab,
};
#[allow(unused_imports)]
use crate::gl_renderer::{BasicRenderer, MultisampleRenderer, RenderFramebuffer, SceneRenderer};
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
    auto_camera: AutoCamera,
    msaa_samples: i32,

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
    image_renderer: Option<ImageRenderer>,
}

impl Default for SaveImageTab {
    fn default() -> Self {
        let auto_camera = AutoCamera::default();

        let mut definition_select_panel = DefinitionSelectPanel::multi_select();
        let selector_observer_id = definition_select_panel.register_observer();

        Self {
            auto_camera,
            msaa_samples: 8,
            definition_select_panel,
            selector_observer_id,
            gl: None,
            scene: Default::default(),
            renderer: None,
            mesh_loaded: false,
            image_renderer: None,
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
                        let canvas_size = utils::fit_size_aspect(
                            ui.available_size(),
                            self.auto_camera.aspect_ratio(),
                        );

                        utils::ui_center(ui, canvas_size, |ui| {
                            egui::Frame::canvas(ui.style())
                                .inner_margin(0.0)
                                .show(ui, |ui| {
                                    let (rect, response) = ui.allocate_exact_size(
                                        egui::vec2(
                                            (canvas_size.x - 2.0).max(1.0),
                                            (canvas_size.y - 2.0).max(1.0),
                                        ),
                                        egui::Sense::drag(),
                                    );
                                    self.auto_camera.control(ui, response);

                                    super::paint_checker_pattern(ui, rect);

                                    if self.image_renderer.is_none() {
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

        if let Some(renderer) = &mut self.image_renderer {
            renderer.update();
            if renderer.progress().done() {
                self.image_renderer = None;
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

    fn ui_camera_params(&mut self, ui: &mut egui::Ui, id: Id) {
        Grid::new(id).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("Image size");
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.auto_camera.width)
                        .range(1..=10000)
                        .suffix("px"),
                );
                ui.label("x");
                ui.add(
                    DragValue::new(&mut self.auto_camera.height)
                        .range(1..=10000)
                        .suffix("px"),
                );
            });
            ui.end_row();

            ui.label("MSAA samples");
            ui.add(DragValue::new(&mut self.msaa_samples).range(0..=32));
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
                        0.0..=((self.auto_camera.width.min(self.auto_camera.height) - 10) / 2)
                            as f32,
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

        let mut mesh_options = BlockViewStateMeshOptions::default();
        if let Some(definition) = self.definition_select_panel.selected_definition() {
            if let Some(meshes) = definition.lock().ok().and_then(|mut d| d.load_meshes()) {
                let options = BlockViewStateMeshOptions::from_definition_meshes(meshes.as_ref());
                mesh_options.or(&options);
            }
        }
        if let Some(selector) = self.definition_select_panel.multi_selector() {
            for s in selector.borrow().selection() {
                if let Ok(mut definition) = s.lock() {
                    if let (_, Some(meshes)) = definition.load_data_meshes() {
                        let options =
                            BlockViewStateMeshOptions::from_definition_meshes(meshes.as_ref());
                        mesh_options.or(&options);
                    }
                }
            }
        }

        let scene_state_changed = self.scene.state_ui(ui, mesh_options);
        if mesh_loaded_now || scene_state_changed {
            self.scene.update(&data, &meshes);
        }
    }

    #[allow(unused_variables)]
    fn ui_buttons(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
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
                    #[cfg(not(target_arch = "wasm32"))]
                    self.save_image(definitions, Some(frame));
                }
            }
        });
    }

    fn ui_progress_modal(&self, ctx: &eframe::egui::Context) {
        if let Some(progress) = self.image_renderer.as_ref().map(|r| r.progress()) {
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
        use super::file_dialog;
        use crate::ui::ImageRenderer;
        use std::cmp::Ordering;

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

            let scene = BlockViewScene::clone_state(&self.scene);
            let renderer = SceneRenderer::new(gl, scene.scene());

            let framebuffer: Box<dyn RenderFramebuffer> = if self.msaa_samples > 0 {
                Box::new(MultisampleRenderer::new(
                    gl.clone(),
                    self.auto_camera.width,
                    self.auto_camera.height,
                    self.msaa_samples,
                ))
            } else {
                Box::new(BasicRenderer::new(
                    gl.clone(),
                    self.auto_camera.width,
                    self.auto_camera.height,
                ))
            };

            self.image_renderer = Some(ImageRenderer::new(
                definitions,
                scene,
                renderer,
                &self.auto_camera,
                framebuffer,
                save_path,
                !is_single,
            ));
        }
    }
}
