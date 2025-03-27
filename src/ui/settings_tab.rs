use super::{file_dialog, tab::Tab, DefinitionsStore, State};
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
    ) {
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
                            self.update_rom_folder(
                                std::path::PathBuf::from(self.rom_path.clone()),
                                state,
                                definitions_store,
                            );
                        }

                        if ui.button("Select").clicked() {
                            if let Some(rom_path) = file_dialog::open_rom_folder_dialog(Some(frame))
                            {
                                self.update_rom_folder(rom_path, state, definitions_store);
                            }
                        }
                    });
                    ui.end_row();
                }
            });
        });
    }
}

impl SettingsTab {
    #[cfg(not(target_arch = "wasm32"))]
    fn update_rom_folder(
        &self,
        rom_path: std::path::PathBuf,
        state: &mut State,
        definitions_store: &mut DefinitionsStore,
    ) {
        // TODO: ここでエラー出たら拾って表示
        let _ = definitions_store.open_rom_directory(Some(rom_path.clone()));
        state.rom_path = Some(rom_path);
    }
}
