use crate::sw_block_definition::{SwBlockDefinition, SwBlockDefinitionMeshKey};
use enum_map::{self, EnumMap};
use std::{fs, io, path::Path};

macro_rules! getter_setter {
    ($target:ident, $name:ident, $setter_name:ident, $type:ty) => {
        impl $target {
            pub fn $name(&self) -> $type {
                self.$name
            }

            pub fn $setter_name(&mut self, value: $type) {
                if (self.$name != value) {
                    self.$name = value;
                    self.changed();
                }
            }
        }
    };
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    definitions: Vec<SwBlockDefinition>,
    selected_definition_index: Option<usize>,
    show_all_attributes: bool,
    hide_default_attributes: bool,
    show_xyz_axis: bool,
    show_surfaces: bool,
    show_surface_edge: bool,
    show_mesh: EnumMap<SwBlockDefinitionMeshKey, bool>,
    #[serde(skip)]
    changed: Option<bool>,
}

impl Default for State {
    fn default() -> Self {
        let mut show_mesh = EnumMap::default();
        for (key, _) in show_mesh {
            show_mesh[key] = true;
        }
        Self {
            definitions: Vec::new(),
            selected_definition_index: None,
            show_all_attributes: false,
            hide_default_attributes: false,
            show_xyz_axis: true,
            show_surfaces: true,
            show_surface_edge: true,
            show_mesh,
            changed: None,
        }
    }
}

impl State {
    pub fn update(&mut self) {
        // 描画フレームごとに1回呼ぶ
        self.changed = Some(false);
    }

    fn changed(&mut self) {
        self.changed = Some(true);
    }

    pub fn is_changed(&self) -> bool {
        self.changed.is_none() || self.changed.unwrap()
    }

    pub fn definitions(&self) -> &Vec<SwBlockDefinition> {
        &self.definitions
    }

    pub fn selected_definition_index(&self) -> &Option<usize> {
        &self.selected_definition_index
    }

    pub fn set_selected_definition_index(&mut self, value: Option<usize>) {
        if self.selected_definition_index != value {
            self.selected_definition_index = value;
            self.changed();
        }
    }

    pub fn show_mesh(&self) -> &EnumMap<SwBlockDefinitionMeshKey, bool> {
        &self.show_mesh
    }

    pub fn set_show_mesh(&mut self, key: SwBlockDefinitionMeshKey, value: bool) {
        if self.show_mesh[key.clone()] != value {
            self.show_mesh[key] = value;
            self.changed();
        }
    }

    pub fn selected_definition(&mut self) -> Option<&mut SwBlockDefinition> {
        self.definitions.get_mut(self.selected_definition_index?)
    }

    pub fn open_rom_directory<P: AsRef<Path>>(&mut self, rom_path: P) -> io::Result<()> {
        // ディレクトリ内の .xml ファイルを列挙
        match fs::read_dir(rom_path.as_ref().join("data\\definitions")) {
            Ok(dir) => {
                self.definitions = dir
                    .filter_map(|entry| {
                        if entry.is_err() {
                            return None;
                        }
                        let entry_path = entry.unwrap().path();
                        if entry_path.is_file() && entry_path.extension()? == "xml" {
                            return SwBlockDefinition::new(&rom_path, entry_path);
                        }
                        None
                    })
                    .collect();
                self.selected_definition_index = None;
                self.changed();
                Ok(())
            }
            Err(err) => {
                self.definitions = Vec::new();
                self.selected_definition_index = None;
                Err(err)
            }
        }
    }
}

getter_setter!(State, show_all_attributes, set_show_all_sttributes, bool);
getter_setter!(
    State,
    hide_default_attributes,
    set_hide_default_attributes,
    bool
);
getter_setter!(State, show_xyz_axis, set_show_xyz_axis, bool);
getter_setter!(State, show_surfaces, set_show_surfaces, bool);
getter_setter!(State, show_surface_edge, set_show_surface_edge, bool);
