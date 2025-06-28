use crate::{store::ModKey, sw_gl_3d::FileLoader};
use std::path::PathBuf;

pub struct ModFileLoader {
    rom_paths: Vec<PathBuf>,
}

impl ModFileLoader {
    pub fn new(rom_paths: Vec<PathBuf>) -> Self {
        Self { rom_paths }
    }

    pub fn from_mod_key(
        value: ModKey,
        rom_path: PathBuf,
        mods_path: PathBuf,
        workshop_path: PathBuf,
    ) -> Self {
        match value {
            ModKey::Stormworks => ModFileLoader::new(vec![rom_path]),
            ModKey::Local(folder_name) => {
                ModFileLoader::new(vec![mods_path.join(folder_name), rom_path])
            }
            ModKey::Workshop(folder_name) => {
                ModFileLoader::new(vec![workshop_path.join(folder_name), rom_path])
            }
        }
    }
}

impl FileLoader for ModFileLoader {
    fn load_file(&self, path: &str) -> Option<PathBuf> {
        for rom_path in &self.rom_paths {
            let path_buf = rom_path.join(path).to_path_buf();
            if path_buf.is_file() {
                return Some(path_buf);
            }
        }
        None
    }
}
