use crate::definition_hub::{BlockDefinition, DefinitionRegistory, ModDefinition, ModKey};
use egui::{vec2, Align, Button, Layout, TextEdit};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

fn search(
    mod_definition: &ModDefinition,
    filename: &str,
    block: &BlockDefinition,
    pat: &str,
) -> bool {
    if mod_definition
        .use_manifest(|manifest| manifest.name.as_ref().map(|n| n.contains(pat)))
        .flatten()
        .unwrap_or(false)
    {
        return true;
    }

    if filename.contains(pat) {
        return true;
    }

    if block
        .use_data(|definition| {
            definition
                .name
                .as_ref()
                .map(|s| s.contains(pat))
                .or_else(|| definition.tags.as_ref().map(|s| s.contains(pat)))
        })
        .flatten()
        .unwrap_or(false)
    {
        return true;
    }

    false
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionSearch {
    search_text: String,
    #[serde(skip)]
    search_result: BTreeMap<(ModKey, String), bool>,
}

impl DefinitionSearch {
    fn clear(&mut self) {
        self.search_text.clear();
        self.search_result.clear();
    }

    pub fn update_search(&mut self, registory: &mut DefinitionRegistory) {
        let pat = self.search_text.to_lowercase();

        for (key, mod_definition, block) in registory.definitions() {
            let key = (key.0.clone(), key.1.clone());
            if self.search_result.contains_key(&key) {
                continue;
            }

            if search(mod_definition, &key.1, block, &pat) {
                self.search_result.insert(key, true);
            }
        }
    }

    pub fn get_result_items(&self) -> impl Iterator<Item = &(ModKey, String)> {
        self.search_result
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
                    self.search_result.clear();
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
        self.inner.borrow().get_result_items().cloned().collect()
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        self.inner.borrow_mut().ui(ui);
    }
}
