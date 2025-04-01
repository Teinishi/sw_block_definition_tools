use super::{
    tab::Tab, DefinitionSingleSelect, DefinitionsStore, MainTab, SaveImageTab, SettingsTab, State,
    TabVariants,
};
use egui::{Sides, TopBottomPanel};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

#[derive(serde::Serialize, serde::Deserialize, Default)]
#[serde(default)]
pub struct MainApp {
    state: State,
    definitions_store: DefinitionsStore,
    search_text: Rc<RefCell<String>>,
    #[serde(skip)]
    selector: Rc<RefCell<DefinitionSingleSelect>>,
    tab: TabVariants,

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
        app.main_tab.use_selector(app.selector.clone());
        app.main_tab.use_search_text(app.search_text.clone());
        app.save_image_tab.creation_context(cc);
        app.save_image_tab.use_selector(app.selector.clone());
        app.save_image_tab.use_search_text(app.search_text.clone());
        app.settings_tab.creation_context(cc);
        app.settings_tab.use_selector(app.selector.clone());
        app.settings_tab.use_search_text(app.search_text.clone());

        app
    }

    fn reset(&mut self) {
        self.state = Default::default();
        self.definitions_store = Default::default();
        self.selector = Default::default();
        self.tab = Default::default();

        self.main_tab.reset();
        self.save_image_tab.reset();
        self.settings_tab.reset();
    }
}

impl eframe::App for MainApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self);
    }

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        self.main_tab.destroy(gl);
        self.save_image_tab.destroy(gl);
        self.settings_tab.destroy(gl);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.state.start_frame(&self.definitions_store);

        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                Sides::new().show(
                    ui,
                    |ui| {
                        ui.selectable_value(&mut self.tab, TabVariants::Main, "Block data");
                        ui.selectable_value(&mut self.tab, TabVariants::SaveImage, "Save image");
                        ui.selectable_value(&mut self.tab, TabVariants::Settings, "Settings");
                    },
                    |ui| {
                        if self.state.loading_state().is_some() {
                            ui.spinner();
                        }
                    },
                )
            });
        });

        let action = match self.tab {
            TabVariants::Main => {
                self.main_tab
                    .update(ctx, frame, &mut self.state, &mut self.definitions_store)
            }
            TabVariants::SaveImage => {
                self.save_image_tab
                    .update(ctx, frame, &mut self.state, &mut self.definitions_store)
            }
            TabVariants::Settings => {
                self.settings_tab
                    .update(ctx, frame, &mut self.state, &mut self.definitions_store)
            }
        };

        self.state.end_frame(ctx);

        if let Some(action) = action {
            match action {
                AppAction::Reset => {
                    self.reset();
                }
                AppAction::SelectRomFolder => {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(rom_path) = super::file_dialog::open_rom_folder_dialog(Some(frame))
                    {
                        update_rom_folder(rom_path, &mut self.state, &mut self.definitions_store);
                    }
                }
                #[allow(unused_variables)]
                AppAction::UpdateRomFolder(rom_path) => {
                    #[cfg(not(target_arch = "wasm32"))]
                    update_rom_folder(rom_path, &mut self.state, &mut self.definitions_store);
                }
            }
        }
    }
}

pub enum AppAction {
    Reset,
    SelectRomFolder,
    UpdateRomFolder(PathBuf),
}

#[cfg(not(target_arch = "wasm32"))]
fn update_rom_folder(
    rom_path: PathBuf,
    state: &mut State,
    definitions_store: &mut DefinitionsStore,
) {
    // TODO: ここでエラー出たら拾って表示
    let _ = definitions_store.open_rom_directory(Some(rom_path.clone()));
    state.rom_path = Some(rom_path);
}
