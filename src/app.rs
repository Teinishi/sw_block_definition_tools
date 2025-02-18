use crate::ui::DefinitionSelectPanel;
use raw_window_handle;
use rfd::FileDialog;
use std::path::Path;

const STORMWORKS_DATA_PATH: &str = "Steam\\steamapps\\common\\Stormworks\\rom\\data";

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct MainApp {
    definition_select_panel: DefinitionSelectPanel,
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MainApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open Definitions Folder").clicked() {
                            self.open_definitions_folder(Some(frame));
                            ui.close_menu();
                        }

                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }

                ui.separator();

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(80.0..=300.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                    self.definition_select_panel.ui(ui);
                });
            });

        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}

impl MainApp {
    fn open_definitions_folder<
        W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
    >(
        &mut self,
        parent: Option<&W>,
    ) {
        let mut dialog = FileDialog::new();
        if let Some(p) = parent {
            dialog = dialog.set_parent(p)
        }
        if let Ok(program_files) = std::env::var("ProgramFiles(x86)") {
            dialog = dialog.set_directory(Path::new(&program_files).join(STORMWORKS_DATA_PATH))
        }
        if let Some(pathbuf) = dialog.pick_folder() {
            let _ = self.definition_select_panel.set_directory(&pathbuf);
        }
    }
}
