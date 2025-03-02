use crate::sw_block_definition::{
    AttributeSpecifier, AttributeValue, SwBlockDefinition, SwBlockDefinitionMeshKey,
    SwBlockDefinitionMeshes,
};
use enum_map::{self, EnumMap};
use std::{fs, io, path::PathBuf};

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
    rom_path: Option<PathBuf>,
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
            rom_path: None,
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

    pub fn rom_path(&self) -> &Option<PathBuf> {
        &self.rom_path
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

    pub fn definition_index(&self, definition: &SwBlockDefinition) -> Option<usize> {
        self.definitions.iter().position(|d| d == definition)
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

    pub fn selected_meshes(&mut self) -> Option<std::rc::Rc<SwBlockDefinitionMeshes>> {
        self.definitions
            .get_mut(self.selected_definition_index?)?
            .load_meshes()
    }

    pub fn open_rom_directory(&mut self, rom_path: PathBuf) -> io::Result<()> {
        // ディレクトリ内の .xml ファイルを列挙
        match fs::read_dir(rom_path.join("data\\definitions")) {
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
                self.rom_path = Some(rom_path);
                self.changed();
                Ok(())
            }
            Err(err) => {
                self.definitions = Vec::new();
                self.selected_definition_index = None;
                self.rom_path = None;
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

impl State {
    pub fn load_all_definitions(&mut self) {
        for definition in &mut self.definitions {
            let _ = definition.load_data();
        }
    }

    pub fn get_attribute_all_definitions(
        &self,
        specifier: &AttributeSpecifier,
    ) -> Vec<(usize, &SwBlockDefinition, AttributeValue)> {
        let mut values: Vec<(usize, &SwBlockDefinition, AttributeValue)> = Vec::new();
        for (i, definition) in self.definitions.iter().enumerate() {
            if let Some(Ok(data)) = definition.data() {
                for value in specifier.get_value_root(&data) {
                    values.push((i, definition, value));
                }
            }
        }
        values
    }
}
