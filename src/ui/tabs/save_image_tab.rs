use super::Tab;
#[cfg(not(target_arch = "wasm32"))]
use crate::definition_hub::ModKey;
use crate::{
    definition_hub::{BlockDefinition, DefinitionRegistory},
    state::State,
    sw_gl_3d::{BasicRenderer, MultisampleRenderer, RenderFramebuffer, SceneRenderer},
    ui::{
        app::BlockSingleSelection,
        components::SharedDefinitionSearch,
        paint_canvas_3d, paint_checker_pattern,
        panels::DefinitionMultiSelectPanel,
        utils::{ui_center, ui_dragvalue_vec_z_inv},
        AppAction, AutoCamera, BlockViewScene, BlockViewStateMeshOptions, ImageRenderer,
    },
    utils::fit_size_aspect,
};
use eframe::glow::Context;
use egui::{
    Button, CentralPanel, DragValue, Frame, Grid, Id, Modal, ProgressBar, ScrollArea, SidePanel,
    Sides, Slider,
};
use egui_extras::{Size, StripBuilder};
use std::{collections::HashSet, sync::Arc};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SaveImageTab {
    auto_camera: AutoCamera,
    msaa_samples: i32,

    #[serde(skip)]
    definition_select_panel: DefinitionMultiSelectPanel,
    #[serde(skip)]
    selection: BlockSingleSelection,

    #[serde(skip)]
    gl: Option<Arc<Context>>,
    scene: BlockViewScene,
    #[serde(skip)]
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    #[serde(skip)]
    mesh_loaded: bool,

    #[serde(skip)]
    image_renderer: Option<ImageRenderer>,

    #[serde(skip)]
    scene_update_done: bool,
}

impl Default for SaveImageTab {
    fn default() -> Self {
        let auto_camera = AutoCamera::default();
        let definition_select_panel = DefinitionMultiSelectPanel::default();
        let selection = definition_select_panel.single_selection().clone();

        Self {
            auto_camera,
            msaa_samples: 8,
            definition_select_panel,
            selection,
            gl: None,
            scene: Default::default(),
            renderer: None,
            mesh_loaded: false,
            image_renderer: None,
            scene_update_done: false,
        }
    }
}

impl Tab for SaveImageTab {
    fn reset(&mut self) {
        self.auto_camera = Default::default();
        self.msaa_samples = 8;
        self.definition_select_panel = Default::default();
        self.scene.reset();
        self.mesh_loaded = false;
        self.image_renderer = None;
    }

    fn creation_context<'a>(
        &mut self,
        cc: &'a eframe::CreationContext<'a>,
        search: SharedDefinitionSearch,
        selection: BlockSingleSelection,
    ) {
        if let Some(gl) = &cc.gl {
            let renderer = SceneRenderer::new(gl, self.scene.scene());
            self.renderer = Some(Arc::new(egui::mutex::Mutex::new(renderer)));
            self.gl = Some(gl.clone());
        }
        self.definition_select_panel.use_search(search);
        self.definition_select_panel
            .use_selection(selection.clone());
        self.selection = selection;
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
        registory: &mut DefinitionRegistory,
    ) -> Option<AppAction> {
        SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(80.0..=500.0)
            .show(ctx, |ui| {
                self.definition_select_panel.ui(ui, registory);
            });

        let key = self.definition_select_panel.single_selection().get();
        let definition = key.as_ref().and_then(|key| registory.get(key)).cloned();
        let data = definition
            .as_ref()
            .and_then(|d| d.load_data())
            .and_then(|d| d.ok());

        // 現在の選択に含まれるmeshの列挙
        let mut mesh_options = BlockViewStateMeshOptions::default();
        if let Some(meshes) = definition.as_ref().and_then(|d| d.load_meshes()) {
            let options = BlockViewStateMeshOptions::from_definition_meshes(meshes.as_ref(), &data);
            mesh_options.or(&options);
        }

        // 複数選択してれば裏で選択しているものも選択肢を出す
        for key in self.definition_select_panel.multiple_selection().get() {
            if let Some(definition) = registory.get(&key) {
                if let (data, Some(meshes)) = definition.load_data_meshes() {
                    let options =
                        BlockViewStateMeshOptions::from_definition_meshes(meshes.as_ref(), &data);
                    mesh_options.or(&options);
                }
            }
        }

        // meshのロード検出
        let mesh_loaded = definition
            .as_ref()
            .map(|d| d.load_meshes().is_some())
            .unwrap_or(false);
        let mesh_loaded_now = mesh_loaded != self.mesh_loaded;
        self.mesh_loaded = mesh_loaded;

        let mut scene_update = false;

        // 画像保存設定UI
        SidePanel::right("save_image_right_panel")
            .resizable(true)
            .default_width(300.0)
            .width_range(80.0..=500.0)
            .show(ctx, |ui| {
                StripBuilder::new(ui)
                    .size(Size::remainder())
                    .size(Size::exact(68.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            ScrollArea::both().show(ui, |ui| {
                                ui.add_space(4.0);

                                self.ui_camera_params(ui, Id::new("save_image_camera_params"));

                                ui.add_space(4.0);
                                ui.separator();
                                ui.add_space(4.0);

                                scene_update =
                                    self.scene.state_ui(ui, &mesh_options) || scene_update;

                                ui.add_space(4.0);
                                ui.separator();
                                ui.add_space(4.0);

                                scene_update =
                                    self.scene.appearance_ui(ui, Id::new("save_image_colors"))
                                        || scene_update;

                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    ui.add_space(4.0);
                                    ui.separator();
                                    ui.add_space(4.0);

                                    scene_update = self.ui_config_file(ui, frame) || scene_update;
                                }
                            });
                        });

                        strip.cell(|ui| {
                            ui.add_space(4.0);
                            self.ui_button(ui, frame, registory);
                        });
                    });
            });

        // 描画領域
        CentralPanel::default().show(ctx, |ui| {
            let canvas_size = fit_size_aspect(ui.available_size(), self.auto_camera.aspect_ratio());

            ui_center(ui, canvas_size, |ui| {
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

                        paint_checker_pattern(ui, rect);

                        if self.image_renderer.is_none() {
                            if let Some(renderer) = &self.renderer {
                                paint_canvas_3d(
                                    ui,
                                    rect,
                                    self.auto_camera.camera.clone(),
                                    renderer.clone(),
                                    self.scene.appearance(),
                                );
                            }
                        }
                    });
            });
        });

        // 描画内容を更新
        if !self.scene_update_done || mesh_loaded_now || scene_update {
            if let Some(definition) = definition.as_ref() {
                self.scene_update_done = self.scene.update(definition, registory);
            }
        }

        // 出力中は進捗を表示
        if let Some(renderer) = &mut self.image_renderer {
            renderer.update(registory, self.scene.state());
            if renderer.progress().done() {
                self.image_renderer = None;
            } else {
                self.ui_progress_modal(ctx);
                ctx.request_repaint();
                return None;
            }
        }

        // 自動カメラ
        if let Some(definition) = definition.as_ref() {
            self.auto_camera
                .update(definition, registory, self.scene.state());
        }

        // 選択変更時に描画内容変更
        if self.selection.check_update() {
            self.update_scene(definition.as_ref(), registory);
        }

        None
    }
}

impl SaveImageTab {
    fn update_scene(
        &mut self,
        definition: Option<&BlockDefinition>,
        registory: &mut DefinitionRegistory,
    ) {
        if let Some(definition) = definition {
            self.scene.update(definition, registory);
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
                ui_dragvalue_vec_z_inv(ui, &mut camera.center, 0.01);
                ui.end_row();

                ui.label("Distance");
                direction_changed |= ui
                    .add(
                        Slider::new(&mut distance, 0.1..=100.0)
                            .drag_value_speed(0.1)
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

    #[cfg(not(target_arch = "wasm32"))]
    fn ui_config_file(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) -> bool {
        let mut scene_update = false;
        StripBuilder::new(ui)
            .sizes(Size::remainder(), 2)
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    let button = ui.add_sized(
                        egui::vec2(ui.available_width(), 20.0),
                        Button::new("Load config"),
                    );
                    if button.clicked() {
                        scene_update = self.load_config(Some(&frame));
                    }
                });
                strip.cell(|ui| {
                    let button = ui.add_sized(
                        egui::vec2(ui.available_width(), 20.0),
                        Button::new("Save config"),
                    );
                    if button.clicked() {
                        self.save_config(Some(&frame));
                    }
                });
            });
        scene_update
    }

    #[allow(unused_variables)]
    fn ui_button(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        registory: &DefinitionRegistory,
    ) {
        let multi_selector = self.definition_select_panel.multiple_selection();
        let count = multi_selector.count();
        let mut save_definitions = None;

        let size = egui::vec2(ui.available_width(), 60.0);

        if count > 0 {
            let button = ui.add_sized(size, Button::new(format!("Save {} images", count)));
            if button.clicked() {
                save_definitions = Some(multi_selector.get());
            }
        } else if let Some((mod_key, definition)) =
            self.definition_select_panel.single_selection().get()
        {
            let button = ui.add_sized(size, Button::new("Save image"));
            if button.clicked() {
                save_definitions = Some(HashSet::from([(mod_key, definition)]));
            }
        }

        if let Some(definitions) = save_definitions {
            #[cfg(not(target_arch = "wasm32"))]
            self.save_image(definitions, registory, Some(frame));
        }
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
    fn load_config<W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        &mut self,
        dialog_parent: Option<&W>,
    ) -> bool {
        use crate::{file_dialog, ui::SaveImageConfig};
        use std::io::BufReader;

        if let Some(path) = file_dialog::load_json_dialog(dialog_parent, Some("config.json")) {
            if let Ok(file) = std::fs::File::open(path) {
                let config: serde_json::Result<SaveImageConfig> =
                    serde_json::from_reader(BufReader::new(file));
                if let Ok(config) = config {
                    config.apply(&mut self.auto_camera, &mut self.scene);
                    return true;
                }
            }
        }
        false
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_config<W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        &self,
        dialog_parent: Option<&W>,
    ) {
        use crate::{file_dialog, ui::SaveImageConfig};
        use std::io::Write;

        let config = SaveImageConfig::new(self.auto_camera.clone(), &self.scene);
        if let Ok(json) = serde_json::to_string(&config) {
            if let Some(path) = file_dialog::save_json_dialog(dialog_parent, Some("config.json")) {
                if let Ok(mut file) = std::fs::File::create(path) {
                    let _ = file.write_all(json.as_bytes());
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_image<W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        &mut self,
        definitions: HashSet<(ModKey, String)>,
        registory: &DefinitionRegistory,
        dialog_parent: Option<&W>,
    ) {
        use crate::{file_dialog, ui::ImageRenderer};
        use std::cmp::Ordering;

        if let Some(gl) = &self.gl {
            // 保存場所のダイアログ
            let (save_path, is_single) = match definitions.len().cmp(&1) {
                Ordering::Greater => (file_dialog::open_folder_dialog(dialog_parent), false),
                Ordering::Equal => (
                    file_dialog::save_png_dialog(
                        dialog_parent,
                        Some(&definitions.iter().next().unwrap().1),
                    ),
                    true,
                ),
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
                definitions
                    .iter()
                    .filter_map(|key| registory.get(key))
                    .cloned()
                    .collect(),
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
