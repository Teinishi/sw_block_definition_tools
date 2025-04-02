#[cfg(not(target_arch = "wasm32"))]
fn dialog<W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
    parent: Option<&W>,
    directory: Option<std::path::PathBuf>,
    filename: Option<&str>,
) -> rfd::FileDialog {
    use rfd::FileDialog;

    let mut dialog = FileDialog::new();
    if let Some(parent) = parent {
        dialog = dialog.set_parent(parent);
    }
    if let Some(directory) = directory {
        dialog = dialog.set_directory(directory);
    }
    if let Some(filename) = filename {
        dialog = dialog.set_file_name(filename);
    }

    dialog
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_rom_folder_dialog<
    W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
>(
    parent: Option<&W>,
) -> Option<std::path::PathBuf> {
    const STORMWORKS_DATA_PATH: &str = "Steam\\steamapps\\common\\Stormworks";

    dialog(
        parent,
        std::env::var("ProgramFiles(x86)")
            .ok()
            .map(|p| std::path::Path::new(&p).join(STORMWORKS_DATA_PATH)),
        Some("rom"),
    )
    .pick_folder()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_png_dialog<
    W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
>(
    parent: Option<&W>,
    filename: Option<&str>,
) -> Option<std::path::PathBuf> {
    dialog(parent, None, filename)
        .add_filter("PNG Image", &["png"])
        .save_file()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_json_dialog<
    W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
>(
    parent: Option<&W>,
    filename: Option<&str>,
) -> Option<std::path::PathBuf> {
    dialog(parent, None, filename)
        .add_filter("JSON", &["json"])
        .save_file()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_folder_dialog<
    W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
>(
    parent: Option<&W>,
) -> Option<std::path::PathBuf> {
    dialog(parent, None, None).pick_folder()
}
