use crate::definition_hub::{BlockDefinition, DefinitionRegistory, ModDefinition, ModKey};
use egui::{vec2, Align, Button, Layout, TextEdit};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

fn search(
    mod_definition: &ModDefinition,
    filename: &str,
    block: &BlockDefinition,
    pat: &str,
) -> Option<bool> {
    // 検索、ロード未完了ならNoneを返す
    if filename.contains(pat) {
        return Some(true);
    }

    let result_manifest = mod_definition.use_manifest(|manifest| {
        manifest
            .name
            .as_ref()
            .map(|n| n.contains(pat))
            .unwrap_or(false)
    });
    match result_manifest {
        Some(true) => return Some(true),
        None => return None,
        _ => {}
    }

    let result_definition = block.use_data(|definition| {
        // name または tags を検索
        definition
            .name
            .as_ref()
            .map(|s| s.contains(pat))
            .or_else(|| definition.tags.as_ref().map(|s| s.contains(pat)))
            .unwrap_or(false)
    });
    match result_definition {
        Some(true) => return Some(true),
        None => return None,
        _ => {}
    }

    None
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionSearch {
    search_text: String,
    #[serde(skip)]
    search_cache: BTreeMap<(ModKey, String), bool>,
    #[serde(skip)]
    last_version: Option<u32>,
}

impl DefinitionSearch {
    fn clear(&mut self) {
        self.search_text.clear();
        self.search_cache.clear();
    }

    pub fn update_search(&mut self, registory: &mut DefinitionRegistory) {
        // registory を見て更新があったらキャッシュリセット
        let current_version = registory.current_version();
        if current_version != self.last_version {
            self.search_cache.clear();
            self.last_version = current_version;
        }

        let pat = self.search_text.to_lowercase();

        // 検索結果キャッシュを埋めていく、未ロードならNone
        for (key, mod_definition, block) in registory.definitions() {
            let key = (key.0.clone(), key.1.clone());
            if self.search_cache.contains_key(&key) {
                continue;
            }

            if let Some(result) = search(mod_definition, &key.1, block, &pat) {
                self.search_cache.insert(key, result);
            }
        }
    }

    pub fn get_result(&self) -> impl Iterator<Item = &(ModKey, String)> {
        self.search_cache
            .iter()
            .filter_map(|(k, v)| if *v { Some(k) } else { None })
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 20.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                if ui
                    .add_sized(vec2(20.0, 20.0), Button::new("\u{274C}"))
                    .clicked()
                {
                    self.clear();
                }

                let search = ui.add_sized(
                    vec2(ui.available_width(), 20.0),
                    TextEdit::singleline(&mut self.search_text).hint_text("Search"),
                );
                if search.changed() {
                    self.search_cache.clear();
                }
            },
        );
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct SharedDefinitionSearch {
    inner: Rc<RefCell<DefinitionSearch>>,
}

impl SharedDefinitionSearch {
    pub fn update_search(&self, registory: &mut DefinitionRegistory) {
        self.inner.borrow_mut().update_search(registory);
    }

    pub fn get_result_items(&self) -> Vec<(ModKey, String)> {
        self.inner.borrow().get_result().cloned().collect()
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        self.inner.borrow_mut().ui(ui);
    }
}
