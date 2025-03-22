use egui::{Slider, TopBottomPanel, ViewportCommand};

use super::{DefinitionsStore, MainTab, State};

#[derive(serde::Serialize, serde::Deserialize)]
enum Tab {
    Main,
    SaveImage,
}

impl Default for Tab {
    fn default() -> Self {
        Self::Main
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MainApp {
    state: State,
    definitions_store: DefinitionsStore,
    tab: Tab,
    main_tab: MainTab,
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
        fonts.font_data.insert(
            "roboto_mono_regular".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../../fonts/RobotoMono-Regular.ttf"
            ))),
        );
        let font_families_proportional = fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap();
        font_families_proportional.insert(0, "roboto_regular".to_owned());
        font_families_proportional.insert(1, "noto_sans_jp_regular".to_owned());
        let font_families_monospace = fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap();
        font_families_monospace.insert(0, "roboto_mono_regular".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            let app: Option<Self> = eframe::get_value(storage, eframe::APP_KEY);
            if let Some(mut app) = app {
                let _ = app.definitions_store.open_rom_directory(None);
                if let Some(rom_path) = app.definitions_store.rom_path() {
                    app.state.set_rom_path(rom_path.clone());
                }
                app.main_tab.creation_context(cc);
                return app;
            }
        }

        let mut main_page = MainTab::new();
        main_page.creation_context(cc);

        Self {
            state: State::default(),
            definitions_store: DefinitionsStore::default(),
            tab: Tab::default(),
            main_tab: main_page,
        }
    }
}

impl eframe::App for MainApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self);
    }

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        self.main_tab.destory(gl);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open rom folder").clicked() {
                            self.open_rom_folder(Some(frame));
                            ui.close_menu();
                        }

                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
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
                        .text("Sound volume"),
                );
                self.state.set_audio_volume(volume);
            });
        });

        match self.tab {
            Tab::Main => {
                self.main_tab
                    .update(ctx, frame, &mut self.state, &mut self.definitions_store)
            }
            Tab::SaveImage => {
                todo!()
            }
        }

        self.state.update(ctx);
    }
}

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
        if let Some(rom_path) = dialog.pick_folder() {
            // TODO: ここでエラー出たら拾って表示
            let _ = self
                .definitions_store
                .open_rom_directory(Some(rom_path.clone()));
            self.state.set_rom_path(rom_path);
        }
    }
}
