use super::Tab;
use crate::{
    store::{DefinitionsStore, State},
    ui::AppAction,
};
use egui::{CentralPanel, Grid, Slider};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SettingsTab {
    rom_path: String,
    mods_path: String,
    workshop_path: String,
}

impl Tab for SettingsTab {
    #[allow(unused_variables)]
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
        state: &mut State,
        definitions_store: &mut DefinitionsStore,
    ) -> Option<AppAction> {
        let mut action = None;

        CentralPanel::default().show(ctx, |ui| {
            Grid::new("settins").show(ui, |ui| {
                ui.label("Theme");
                egui::widgets::global_theme_preference_buttons(ui);
                ui.end_row();

                ui.label("Sound volume");
                ui.add(
                    Slider::new(&mut state.audio_volume, 0.0..=1.0)
                        .show_value(false)
                        .trailing_fill(true),
                );
                ui.end_row();

                #[cfg(not(target_arch = "wasm32"))]
                {
                    use crate::ui::components::{ui_filepath_edit, FilepathEditAction};

                    ui.label("rom folder");
                    match ui_filepath_edit(ui, &state.rom_path, &mut self.rom_path) {
                        Some(FilepathEditAction::Select) => {
                            action = Some(AppAction::SelectRomFolder)
                        }
                        Some(FilepathEditAction::Update(pathbuf)) => {
                            action = Some(AppAction::UpdateRomFolder(pathbuf))
                        }
                        _ => {}
                    }
                    ui.end_row();

                    ui.label("mods folder");
                    match ui_filepath_edit(ui, &state.mods_path, &mut self.mods_path) {
                        Some(FilepathEditAction::Select) => {
                            action = Some(AppAction::SelectModsFolder)
                        }
                        Some(FilepathEditAction::Update(pathbuf)) => {
                            action = Some(AppAction::UpdateModsFolder(pathbuf))
                        }
                        _ => {}
                    }
                    ui.end_row();

                    ui.label("workshop folder (573090)");
                    match ui_filepath_edit(ui, &state.workshop_path, &mut self.workshop_path) {
                        Some(FilepathEditAction::Select) => {
                            action = Some(AppAction::SelectWorkshopFolder)
                        }
                        Some(FilepathEditAction::Update(pathbuf)) => {
                            action = Some(AppAction::UpdateWorkshopFolder(pathbuf))
                        }
                        _ => {}
                    }
                    ui.end_row();
                }

                if ui.button("Reset").clicked() {
                    action = Some(AppAction::Reset);
                }
            });
        });

        action
    }
}
