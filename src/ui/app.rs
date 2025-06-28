use super::{
    set_fonts,
    tabs::{MainTab, SaveImageTab, SettingsTab, Tab, TabVariants},
};
use crate::{
    definition_hub::{DefinitionRegistory, ModDefinition, ModKey},
    state::State,
    ui::{components::SharedDefinitionSearch, SharedMultipleSelection, SharedSingleSelection},
};
use egui::{Sides, TopBottomPanel};
use std::path::Path;

pub type BlockKey = (ModKey, String);
pub type BlockSingleSelection = SharedSingleSelection<BlockKey>;
pub type BlockMultipleSelection = SharedMultipleSelection<BlockKey>;

#[derive(serde::Serialize, serde::Deserialize, Default)]
#[serde(default)]
pub struct MainApp {
    state: State,
    registory: DefinitionRegistory,
    search: SharedDefinitionSearch,
    #[serde(skip)]
    selection: BlockSingleSelection,
    tab: TabVariants,

    main_tab: MainTab,
    save_image_tab: SaveImageTab,
    settings_tab: SettingsTab,
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_fonts(cc);

        let mut app = cc
            .storage
            .and_then(|storage| {
                // 状態を復元
                let app: Option<Self> = eframe::get_value(storage, eframe::APP_KEY);
                app
            })
            .map(|mut app| {
                app.registory.init(&app.state);
                app
            })
            .unwrap_or_default();

        app.main_tab
            .creation_context(cc, app.search.clone(), app.selection.clone());
        app.save_image_tab
            .creation_context(cc, app.search.clone(), app.selection.clone());
        app.settings_tab
            .creation_context(cc, app.search.clone(), app.selection.clone());

        app
    }

    pub fn reset(&mut self) {
        self.state = Default::default();
        self.registory = Default::default();
        self.search = Default::default();
        self.selection = Default::default();
        self.tab = Default::default();

        self.main_tab.reset();
        self.save_image_tab.reset();
        self.settings_tab.reset();
    }

    pub fn set_rom_folder<P: AsRef<Path>>(&mut self, path: P) {
        self.state.rom_path = Some(path.as_ref().to_path_buf());
        self.registory
            .add_mod(ModDefinition::new(ModKey::Stormworks, path));
    }

    pub fn set_mods_folder<P: AsRef<Path>>(&mut self, path: P) {
        self.state.mods_path = Some(path.as_ref().to_path_buf());
        let _ = self.registory.add_mods_in_folder(path, ModKey::Local);
    }

    pub fn set_workshop_folder<P: AsRef<Path>>(&mut self, path: P) {
        self.state.workshop_path = Some(path.as_ref().to_path_buf());
        let _ = self.registory.add_mods_in_folder(path, ModKey::Workshop);
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
        // self.state.start_frame(&self.registory);

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
                    .update(ctx, frame, &mut self.state, &mut self.registory)
            }
            TabVariants::SaveImage => {
                self.save_image_tab
                    .update(ctx, frame, &mut self.state, &mut self.registory)
            }
            TabVariants::Settings => {
                self.settings_tab
                    .update(ctx, frame, &mut self.state, &mut self.registory)
            }
        };

        self.state.end_frame(ctx);

        if let Some(action) = action {
            action.execute(self, frame);
        }
    }
}
