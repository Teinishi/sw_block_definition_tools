use super::MainApp;
use std::path::PathBuf;

pub enum AppAction {
    Reset,
    SelectRomFolder,
    UpdateRomFolder(PathBuf),
    SelectModsFolder,
    UpdateModsFolder(PathBuf),
    SelectWorkshopFolder,
    UpdateWorkshopFolder(PathBuf),
}

impl AppAction {
    pub fn execute(&self, app: &mut MainApp, frame: &mut eframe::Frame) {
        match self {
            AppAction::Reset => {
                app.reset();
            }
            AppAction::SelectRomFolder => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(path) = crate::file_dialog::open_rom_folder_dialog(Some(frame)) {
                    app.set_rom_folder(path);
                }
            }
            #[allow(unused_variables)]
            AppAction::UpdateRomFolder(path) => {
                #[cfg(not(target_arch = "wasm32"))]
                app.set_rom_folder(path);
            }
            AppAction::SelectModsFolder => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(path) = crate::file_dialog::open_mods_folder_dialog(Some(frame)) {
                    app.set_mods_folder(path);
                }
            }
            #[allow(unused_variables)]
            AppAction::UpdateModsFolder(path) => {
                #[cfg(not(target_arch = "wasm32"))]
                app.set_mods_folder(path);
            }
            AppAction::SelectWorkshopFolder => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(path) = crate::file_dialog::open_workshop_folder_dialog(Some(frame)) {
                    app.set_workshop_folder(path);
                }
            }
            #[allow(unused_variables)]
            AppAction::UpdateWorkshopFolder(path) => {
                #[cfg(not(target_arch = "wasm32"))]
                app.set_workshop_folder(path);
            }
        }
    }
}
