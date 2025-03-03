use super::{
    AttributeDetailWindow, BottomPanel, Definition3dPanel, DefinitionDetailPanel,
    DefinitionSelectPanel, State,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MainApp {
    state: State,
    definition_select_panel: DefinitionSelectPanel,
    definition_detail_panel: DefinitionDetailPanel,
    definition_3d_panel: Option<Definition3dPanel>,
    bottom_panel: BottomPanel,
    window_id: i32,
    attribute_detail_windows: Vec<AttributeDetailWindow>,
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "noto_sans_jp_regular".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../../fonts/NotoSansJP-Regular.ttf"
            ))),
        );
        fonts.font_data.insert(
            "roboto_regular".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../../fonts/Roboto-Regular.ttf"
            ))),
        );
        let font_families = fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap();
        font_families.insert(0, "roboto_regular".to_owned());
        font_families.insert(1, "noto_sans_jp_regular".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            let app: Option<Self> = eframe::get_value(storage, eframe::APP_KEY);
            if let Some(mut app) = app {
                if let Some(definition_3d_panel) = &mut app.definition_3d_panel {
                    definition_3d_panel.creation_context(cc);
                }
                return app;
            }
        }

        Self {
            state: State::default(),
            definition_select_panel: DefinitionSelectPanel::default(),
            definition_detail_panel: DefinitionDetailPanel::default(),
            definition_3d_panel: Definition3dPanel::new(cc, None),
            bottom_panel: BottomPanel::default(),
            window_id: 0,
            attribute_detail_windows: Vec::new(),
        }
    }

    pub fn add_attribute_detail_window(&mut self, mut window: AttributeDetailWindow) {
        window.set_id(egui::Id::new(format!("window_{}", self.window_id)));
        self.attribute_detail_windows.push(window);
        self.window_id += 1;
    }
}

impl eframe::App for MainApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self);
    }

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        if let Some(definition_3d_panel) = &self.definition_3d_panel {
            definition_3d_panel.destroy(gl);
        }
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        for window in &mut self.attribute_detail_windows {
            window.ui(ctx, &mut self.state);
        }
        self.attribute_detail_windows.retain(|w| w.is_open());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open Rom Folder").clicked() {
                            self.open_rom_folder(Some(frame));
                            ui.close_menu();
                        }

                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.separator();
                }

                egui::widgets::global_theme_preference_buttons(ui);

                ui.separator();

                let mut volume = self.state.audio_volume();
                ui.add(
                    Slider::new(&mut volume, 0.0..=1.0)
                        .show_value(false)
                        .trailing_fill(true)
                        .text("Sound Volume"),
                );
                self.state.set_audio_volume(volume);
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(80.0..=500.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(4.0);
                    ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                    self.definition_select_panel.ui(ui, &mut self.state);
                    ui.add_space(4.0);
                });
            });

        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(300.0)
            .width_range(80.0..=800.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(4.0);
                    if let Some(definition_3d_panel) = &mut self.definition_3d_panel {
                        definition_3d_panel.ui(ui, &mut self.state);
                    }
                    ui.add_space(4.0);
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                self.bottom_panel.ui(ui, &mut self.state);
                ui.add_space(4.0);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                if let Some(w) = self.definition_detail_panel.ui(ui, &mut self.state) {
                    self.add_attribute_detail_window(w);
                }
                ui.add_space(10.0);
            });
        });

        self.state.update(ctx);
    }
}

use egui::Slider;
#[cfg(not(target_arch = "wasm32"))]
use raw_window_handle;

#[cfg(not(target_arch = "wasm32"))]
const STORMWORKS_DATA_PATH: &str = "Steam\\steamapps\\common\\Stormworks";

#[cfg(not(target_arch = "wasm32"))]
impl MainApp {
    fn open_rom_folder<
        W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
    >(
        &mut self,
        parent: Option<&W>,
    ) {
        use rfd::FileDialog;
        use std::path::Path;

        let mut dialog = FileDialog::new();
        if let Some(p) = parent {
            dialog = dialog.set_parent(p)
        }
        if let Ok(program_files) = std::env::var("ProgramFiles(x86)") {
            dialog = dialog.set_directory(Path::new(&program_files).join(STORMWORKS_DATA_PATH))
        }
        if let Some(pathbuf) = dialog.pick_folder() {
            let _ = self.state.open_rom_directory(pathbuf);
        }
    }
}
