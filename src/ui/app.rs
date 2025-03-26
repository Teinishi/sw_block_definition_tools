use egui::TopBottomPanel;

use super::{DefinitionsStore, MainTab, SaveImageTab, SettingsTab, State};

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
enum Tab {
    Main,
    SaveImage,
    Settings,
}

impl Default for Tab {
    fn default() -> Self {
        Self::Main
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct MainApp {
    state: State,
    definitions_store: DefinitionsStore,
    tab: Tab,
    main_tab: MainTab,
    save_image_tab: SaveImageTab,
    settings_tab: SettingsTab,
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
                    app.state.rom_path = Some(rom_path.clone());
                }
            }
        }

        let mut app = cc
            .storage
            .and_then(|storage| {
                let app: Option<Self> = eframe::get_value(storage, eframe::APP_KEY);
                app
            })
            .map(|mut app| {
                let _ = app.definitions_store.open_rom_directory(None);
                if let Some(rom_path) = app.definitions_store.rom_path() {
                    app.state.rom_path = Some(rom_path.clone());
                }
                app
            })
            .unwrap_or_default();

        app.main_tab.creation_context(cc);
        app.save_image_tab.creation_context(cc);

        app
    }
}

impl eframe::App for MainApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self);
    }

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        self.main_tab.destory(gl);
        self.save_image_tab.destroy(gl);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.selectable_value(&mut self.tab, Tab::Main, "Block data");
                ui.selectable_value(&mut self.tab, Tab::SaveImage, "Save image");
                ui.selectable_value(&mut self.tab, Tab::Settings, "Settings");
            });
        });

        match self.tab {
            Tab::Main => {
                self.main_tab
                    .update(ctx, frame, &mut self.state, &mut self.definitions_store)
            }
            Tab::SaveImage => {
                self.save_image_tab
                    .update(ctx, frame, &mut self.state, &mut self.definitions_store)
            }
            Tab::Settings => {
                self.settings_tab
                    .update(ctx, frame, &mut self.state, &mut self.definitions_store)
            }
        }

        self.state.update(ctx);
    }
}
