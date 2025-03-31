use super::{AppAction, DefinitionsStore, State, Tab};
use egui::{CentralPanel, Grid, Slider};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SettingsTab {
    rom_path: String,
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
                    ui.label("rom folder");
                    ui.horizontal(|ui| {
                        let rom_path_buf = state.rom_path.clone().unwrap_or_default();
                        self.rom_path = rom_path_buf
                            .as_os_str()
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                        let text_edit = ui.add_sized(
                            [300.0, 18.0],
                            egui::TextEdit::singleline(&mut self.rom_path),
                        );
                        if text_edit.changed() {
                            action = Some(AppAction::UpdateRomFolder(std::path::PathBuf::from(
                                self.rom_path.clone(),
                            )));
                        }

                        if ui.button("Select").clicked() {
                            action = Some(AppAction::SelectRomFolder);
                        }
                    });
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
