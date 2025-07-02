use crate::{
    definition_hub::{DefinitionRegistory, ModKey},
    ui::{
        app::{BlockMultipleSelection, BlockSingleSelection},
        components::SharedDefinitionSearch,
        BlockKey, Selection,
    },
    value_tracker::CheckUpdate,
};
use egui::{
    Align2, Button, Checkbox, Label, Layout, RichText, Sense, Sides, Stroke, UiBuilder, Widget,
};
use egui_extras::{Size, StripBuilder};
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct DefinitionSelectPanel {
    search: SharedDefinitionSearch,
    selection: BlockSingleSelection,
    mod_collapsing: ModCollapsing,
}

impl DefinitionSelectPanel {
    pub fn use_search(&mut self, search: SharedDefinitionSearch) {
        self.search = search;
    }

    pub fn use_selection(&mut self, selection: BlockSingleSelection) {
        self.selection = selection;
    }

    pub fn use_mod_collapsing(&mut self, mod_collapsing: ModCollapsing) {
        self.mod_collapsing = mod_collapsing;
    }

    pub fn selection(&self) -> &BlockSingleSelection {
        &self.selection
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, registory: &mut DefinitionRegistory) {
        ui_panel(
            ui,
            registory,
            &self.search,
            &self.selection,
            None,
            &self.mod_collapsing,
        );
        self.search.update_search(registory);
    }
}

#[derive(Default)]
pub struct DefinitionMultiSelectPanel {
    search: SharedDefinitionSearch,
    single_selection: BlockSingleSelection,
    multiple_selection: BlockMultipleSelection,
    mod_collapsing: ModCollapsing,
    pub auto_select: bool,
    last_selection_version: Option<u32>,
}

impl DefinitionMultiSelectPanel {
    pub fn use_search(&mut self, search: SharedDefinitionSearch) {
        self.search = search;
    }

    pub fn use_selection(&mut self, selection: BlockSingleSelection) {
        self.single_selection = selection;
    }

    pub fn use_mod_collapsing(&mut self, mod_collapsing: ModCollapsing) {
        self.mod_collapsing = mod_collapsing;
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
            &self.mod_collapsing,
        );
        self.search.update_search(registory);

        if self.auto_select
            && !self.multiple_selection.is_empty()
            && self
                .single_selection
                .check_update(&mut self.last_selection_version)
        {
            if let Some(key) = self.single_selection.get().borrow().as_ref() {
                self.multiple_selection.add(key.clone());
            }
        }
    }
}

fn ui_panel(
    ui: &mut egui::Ui,
    registory: &mut DefinitionRegistory,
    search: &SharedDefinitionSearch,
    single_selection: &BlockSingleSelection,
    multi_selection: Option<&BlockMultipleSelection>,
    mod_collapsing: &ModCollapsing,
) {
    ui.add_space(6.0);

    search.ui(ui);

    let items = search.get_result_items();
    if let Some(multi_selection) = &multi_selection {
        ui_select_all(ui, &items, multi_selection);
    }
    ui.add_space(6.0);
    ui_list(
        ui,
        registory,
        &items,
        single_selection,
        multi_selection,
        mod_collapsing,
    );
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
    registory: &mut DefinitionRegistory,
    items: &[BlockKey],
    single_selection: &BlockSingleSelection,
    multi_selection: Option<&BlockMultipleSelection>,
    mod_collapsing: &ModCollapsing,
) {
    let mut items_map: BTreeMap<&ModKey, Vec<&String>> = BTreeMap::new();
    for (mod_key, filename) in items {
        items_map.entry(mod_key).or_default().push(filename);
    }
    let total_len = items_map
        .iter()
        .map(|(k, v)| {
            if mod_collapsing.get(k) {
                1
            } else {
                1 + v.len()
            }
        })
        .sum();

    egui::ScrollArea::vertical().show(ui, |ui| {
        StripBuilder::new(ui)
            .sizes(Size::initial(20.0), total_len)
            .vertical(|mut strip| {
                for (mod_key, filenames) in items_map {
                    let mod_name = match mod_key {
                        ModKey::Stormworks => "Stormworks".to_string(),
                        _ => registory
                            .mods()
                            .get(mod_key)
                            .and_then(|m| m.use_manifest(|m| m.name.clone()))
                            .flatten()
                            .unwrap_or(mod_key.get_folder_name()),
                    };

                    strip.cell(|ui| {
                        let mut collapse = mod_collapsing.get(mod_key);
                        if ui_list_mod_label(ui, &mod_name, &mut collapse) {
                            if let Some(mod_definition) = registory.mods_mut().get_mut(mod_key) {
                                mod_definition.refresh();
                            }
                        }
                        mod_collapsing.set(mod_key.clone(), collapse);
                    });

                    if mod_collapsing.get(mod_key) {
                        continue;
                    }

                    for filename in filenames {
                        strip.strip(|builder| {
                            ui_list_item(
                                builder,
                                mod_key,
                                filename,
                                single_selection,
                                multi_selection,
                            );
                        });
                    }
                }
            });
    });

    mod_collapsing.apply();
}

fn ui_list_mod_label(ui: &mut egui::Ui, mod_name: &str, collapse: &mut bool) -> bool {
    let mut refresh = false;
    let res = ui
        .scope_builder(UiBuilder::new().sense(Sense::click()), |ui| {
            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                0.0,
                ui.visuals().widgets.inactive.bg_fill,
            );
            Sides::new().show(
                ui,
                |ui| {
                    ui.add_space(4.0);
                    ui_collapsing_icon(ui, !*collapse);
                    Label::new(mod_name).selectable(false).ui(ui);
                },
                |ui| {
                    ui.add_space(4.0);
                    let res = ui.add_sized(
                        egui::vec2(16.0, 16.0),
                        Button::new("\u{1F503}").stroke(Stroke::NONE),
                    );
                    if res.clicked() {
                        refresh = true;
                    }
                },
            );
        })
        .response;
    if res.clicked() {
        *collapse = !*collapse;
    }
    refresh
}

fn ui_collapsing_icon(ui: &mut egui::Ui, open: bool) {
    let angle = (if open { 90.0 } else { 0f32 }).to_radians();
    let (res, painter) = ui.allocate_painter([20.0, 20.0].into(), Sense::empty());
    let s = ui.ctx().fonts(|f| {
        let mut t = egui::Shape::text(
            f,
            res.rect.center(),
            Align2::CENTER_CENTER,
            "\u{25B6}",
            egui::FontId::new(12.0, egui::FontFamily::Proportional),
            ui.visuals().text_color(),
        );
        if let egui::epaint::Shape::Text(ts) = &mut t {
            // with_angle_and_anchor が来るまでの暫定対応
            let mut new = ts.clone().with_angle(angle);
            let a0 = new.galley.rect.center().to_vec2();
            let a1 = egui::emath::Rot2::from_angle(angle) * a0;
            new.pos += a0 - a1;

            *ts = new;
        };
        t
    });
    painter.add(s);
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

            if let Some(multi_selection) = multi_selection {
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct ModCollapsing {
    inner: Arc<Mutex<HashMap<ModKey, bool>>>,
    #[serde(skip)]
    scheduled: Arc<Mutex<HashMap<ModKey, bool>>>,
}

impl Clone for ModCollapsing {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            scheduled: self.scheduled.clone(),
        }
    }
}

impl ModCollapsing {
    pub fn get(&self, k: &ModKey) -> bool {
        *self.inner.lock().unwrap().get(k).unwrap_or(&false)
    }

    pub fn set_immediate(&self, k: ModKey, v: bool) {
        self.inner.lock().unwrap().insert(k, v);
    }

    pub fn set(&self, k: ModKey, v: bool) {
        // 変更を一旦保留
        self.scheduled.lock().unwrap().insert(k, v);
    }

    pub fn apply(&self) {
        for (k, v) in self.scheduled.lock().unwrap().drain() {
            self.set_immediate(k, v);
        }
    }
}
