use crate::{
    definition_hub::{DefinitionRegistory, ModKey},
    ui::{
        app::{BlockMultipleSelection, BlockSingleSelection},
        components::SharedDefinitionSearch,
        BlockKey, Selection,
    },
};
use egui::{Checkbox, Layout, RichText};
use egui_extras::{Size, StripBuilder};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct DefinitionSelectPanel {
    search: SharedDefinitionSearch,
    selection: BlockSingleSelection,
}

impl DefinitionSelectPanel {
    pub fn use_search(&mut self, search: SharedDefinitionSearch) {
        self.search = search;
    }

    pub fn use_selection(&mut self, selection: BlockSingleSelection) {
        self.selection = selection;
    }

    pub fn selection(&self) -> &BlockSingleSelection {
        &self.selection
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, registory: &mut DefinitionRegistory) {
        ui_panel(ui, registory, &self.search, &self.selection, None);
        self.search.update_search(registory);
    }
}

#[derive(Default)]
pub struct DefinitionMultiSelectPanel {
    search: SharedDefinitionSearch,
    single_selection: BlockSingleSelection,
    multiple_selection: BlockMultipleSelection,
    pub auto_select: bool,
}

impl DefinitionMultiSelectPanel {
    pub fn use_search(&mut self, search: SharedDefinitionSearch) {
        self.search = search;
    }

    pub fn use_selection(&mut self, selection: BlockSingleSelection) {
        self.single_selection = selection;
    }

    pub fn single_selection(&self) -> &BlockSingleSelection {
        &self.single_selection
    }

    pub fn multiple_selection(&self) -> &BlockMultipleSelection {
        &self.multiple_selection
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, registory: &mut DefinitionRegistory) {
        ui_panel(
            ui,
            registory,
            &self.search,
            &self.single_selection,
            Some(&self.multiple_selection),
        );
        self.search.update_search(registory);

        let select_updated = self.single_selection.check_update();
        if self.auto_select && !self.multiple_selection.is_empty() && select_updated {
            if let Some(selected) = self.single_selection.get() {
                self.multiple_selection.add(selected.clone());
            }
        }
    }
}

fn ui_panel(
    ui: &mut egui::Ui,
    registory: &DefinitionRegistory,
    search: &SharedDefinitionSearch,
    single_selection: &BlockSingleSelection,
    multi_selection: Option<&BlockMultipleSelection>,
) {
    ui.add_space(6.0);

    search.ui(ui);

    let items = search.get_result_items();
    if let Some(multi_selection) = multi_selection {
        ui_select_all(ui, &items, multi_selection);
    }
    ui.add_space(6.0);
    ui_list(ui, registory, &items, single_selection);
}

fn ui_select_all(ui: &mut egui::Ui, items: &[BlockKey], multi_selection: &BlockMultipleSelection) {
    let count = multi_selection.count();
    let mut checked_any = count > 0;
    let checked_all = count >= items.len();
    let indeterminate = if items.is_empty() {
        false
    } else {
        checked_any != checked_all
    };

    if ui
        .add(
            Checkbox::new(&mut checked_any, format!("{} selected", count))
                .indeterminate(indeterminate),
        )
        .changed()
    {
        if checked_any {
            multi_selection.set_selection(items.iter().cloned());
        } else {
            multi_selection.clear();
        }
    }
}

fn ui_list(
    ui: &mut egui::Ui,
    registory: &DefinitionRegistory,
    items: &[BlockKey],
    single_selection: &BlockSingleSelection,
) {
    let mut items_map: BTreeMap<&ModKey, Vec<&String>> = BTreeMap::new();
    for (mod_key, filename) in items {
        items_map.entry(mod_key).or_default().push(filename);
    }
    let total_len = items_map.values().map(|v| 1 + v.len()).sum();

    egui::ScrollArea::vertical().show(ui, |ui| {
        StripBuilder::new(ui)
            .sizes(Size::initial(20.0), total_len)
            .vertical(|mut strip| {
                for (mod_key, filenames) in items_map {
                    let mod_name = match mod_key {
                        ModKey::Stormworks => "Stormworks",
                        _ => &registory
                            .mods
                            .get(mod_key)
                            .and_then(|m| m.manifest.get())
                            .and_then(|m| m.ok())
                            .map(|m| m.name.clone())
                            .unwrap(),
                    };

                    strip.cell(|ui| {
                        ui_list_mod_label(ui, mod_name);
                    });

                    for filename in filenames {
                        strip.strip(|builder| {
                            ui_list_item(builder, mod_key, filename, single_selection, None);
                        });
                    }
                }
            });
    });
}

fn ui_list_mod_label(ui: &mut egui::Ui, mod_name: &str) {
    ui.painter().rect_filled(
        ui.available_rect_before_wrap(),
        0.0,
        ui.visuals().widgets.inactive.bg_fill,
    );
    ui.horizontal_centered(|ui| {
        ui.add_space(4.0);
        ui.label(mod_name);
    });
}

fn ui_list_item(
    builder: StripBuilder<'_>,
    mod_key: &ModKey,
    filename: &str,
    single_selection: &BlockSingleSelection,
    multi_selection: Option<&BlockMultipleSelection>,
) {
    let key = (mod_key.clone(), filename.to_string());

    let builder = if multi_selection.is_some() {
        builder.size(Size::exact(12.0))
    } else {
        builder
    };

    builder
        .size(Size::remainder())
        .cell_layout(Layout::top_down_justified(egui::Align::LEFT))
        .clip(true)
        .horizontal(|mut strip| {
            let mut text = RichText::new(filename);

            if let Some(multi_selection) = &multi_selection {
                if !multi_selection.is_empty() && !multi_selection.is_selected(&key) {
                    text = text.weak();
                }
                let mut is_selected = multi_selection.is_selected(&key);
                let mut toggle = false;

                strip.cell(|ui| {
                    if ui.checkbox(&mut is_selected, "").changed() {
                        toggle = true;
                    }
                });

                if toggle {
                    multi_selection.toggle(key.clone());
                }
            }

            strip.cell(|ui| {
                let selectable_label =
                    ui.selectable_label(single_selection.is_selected(&key), text);
                if selectable_label.clicked() {
                    single_selection.add(key);
                }
            });
        });
}
