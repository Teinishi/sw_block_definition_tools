use super::{
    definitions_store::DefinitionPointer, DefinitionMultiSelect, DefinitionSelect,
    DefinitionSingleSelect, DefinitionsStore,
};
use egui::{vec2, Align, Button, Checkbox, Layout, RichText, TextEdit};
use egui_extras::{Size, StripBuilder};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

pub struct DefinitionSelectPanel {
    search_text: String,
    selector: Rc<RefCell<DefinitionSingleSelect>>,
    selector_observer_id: u32,
    multi_selector: Option<Rc<RefCell<DefinitionMultiSelect>>>,
    search_result: BTreeMap<String, bool>,
    pub auto_select: bool,
}

impl Default for DefinitionSelectPanel {
    fn default() -> Self {
        let mut selector = DefinitionSingleSelect::default();
        let observer_id = selector.register_observer();

        Self {
            search_text: Default::default(),
            selector: Rc::new(RefCell::new(selector)),
            selector_observer_id: observer_id,
            multi_selector: None,
            search_result: Default::default(),
            auto_select: false,
        }
    }
}

impl DefinitionSelectPanel {
    pub fn multi_select() -> Self {
        Self {
            multi_selector: Some(Rc::new(RefCell::new(DefinitionMultiSelect::default()))),
            auto_select: true,
            ..Default::default()
        }
    }

    pub fn use_selector(&mut self, selector: Rc<RefCell<DefinitionSingleSelect>>) {
        self.selector_observer_id = selector.borrow_mut().register_observer();
        self.selector = selector;
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, definitions_store: &mut DefinitionsStore) {
        ui.add_space(6.0);

        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 20.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                if ui
                    .add_sized(vec2(20.0, 20.0), Button::new("\u{274C}"))
                    .clicked()
                {
                    self.search_text.clear();
                }

                let search = ui.add_sized(
                    egui::vec2(ui.available_width(), 20.0),
                    TextEdit::singleline(&mut self.search_text).hint_text("Search"),
                );
                if search.changed() {
                    self.reset_search();
                }
            },
        );

        {
            let no_search = self.search_text.is_empty();
            let binding = definitions_store.definitions().borrow();
            let items: Vec<(&String, &DefinitionPointer)> = binding
                .iter()
                .filter(|(filename, _)| {
                    no_search || self.search_result.get(*filename) == Some(&true)
                })
                .collect();

            if let Some(multi_select) = &self.multi_selector {
                let count = multi_select.borrow().count();
                let mut checked_any = count > 0;
                let checked_all = count >= items.len();
                let indeterminate = checked_any != checked_all;

                if ui
                    .add(
                        Checkbox::new(&mut checked_any, format!("{} selected", count))
                            .indeterminate(indeterminate),
                    )
                    .changed()
                {
                    if checked_any {
                        multi_select
                            .borrow_mut()
                            .set_selection(items.iter().map(|(_, ptr)| *ptr));
                    } else {
                        multi_select.borrow_mut().clear();
                    }
                }
            }

            ui.add_space(6.0);

            self.ui_list(ui, &items);
        }

        self.update_search(definitions_store);

        let select_updated = self
            .selector
            .borrow_mut()
            .check_update(self.selector_observer_id)
            .unwrap_or(false);
        if let Some(multi_selector) = &self.multi_selector {
            if self.auto_select && multi_selector.borrow().count() > 0 && select_updated {
                if let Some(selected) = self.selector.borrow().selected() {
                    multi_selector.borrow_mut().select(&selected);
                }
            }
        }
    }

    fn ui_list(&self, ui: &mut egui::Ui, items: &Vec<(&String, &DefinitionPointer)>) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            StripBuilder::new(ui)
                .sizes(Size::initial(20.0), items.len())
                .vertical(|mut strip| {
                    for (filename, definition) in items {
                        strip.strip(|builder| {
                            self.ui_list_item(builder, filename, definition);
                        });
                    }
                });
        });
    }

    fn ui_list_item(
        &self,
        builder: StripBuilder<'_>,
        filename: &str,
        definition: &DefinitionPointer,
    ) {
        let builder = if self.multi_selector.is_some() {
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

                if let Some(multi_selector) = &self.multi_selector {
                    if multi_selector.borrow().count() > 0
                        && !multi_selector.borrow().is_selected(definition)
                    {
                        text = text.weak();
                    }
                    let mut is_selected = multi_selector.borrow().is_selected(definition);

                    strip.cell(|ui| {
                        if ui.checkbox(&mut is_selected, "").changed() {
                            if is_selected {
                                multi_selector.borrow_mut().select(definition);
                            } else {
                                multi_selector.borrow_mut().unselect(definition);
                            }
                        }
                    });
                }

                strip.cell(|ui| {
                    let selectable_label =
                        ui.selectable_label(self.selector.borrow().is_selected(definition), text);
                    if selectable_label.clicked() {
                        self.selector.borrow_mut().select(definition);
                    }
                });
            });
    }

    pub fn selector(&self) -> Rc<RefCell<DefinitionSingleSelect>> {
        self.selector.clone()
    }

    pub fn multi_selector(&self) -> Option<Rc<RefCell<DefinitionMultiSelect>>> {
        self.multi_selector.clone()
    }

    pub fn selected_definition(&self) -> Option<DefinitionPointer> {
        self.selector.borrow().selected()
    }

    pub fn register_observer(&mut self) -> u32 {
        self.selector.borrow_mut().register_observer()
    }

    pub fn check_update(&mut self, observer_id: u32) -> Option<bool> {
        self.selector.borrow_mut().check_update(observer_id)
    }

    fn reset_search(&mut self) {
        self.search_result.clear();
    }

    fn update_search(&mut self, definitions_store: &mut DefinitionsStore) {
        for (filename, definition) in definitions_store.definitions().borrow().iter() {
            if self.search_result.contains_key(filename) {
                continue;
            }
            if let Some(result) = definition
                .lock()
                .ok()
                .and_then(|mut d| d.search(&self.search_text))
            {
                self.search_result.insert(filename.clone(), result);
            }
        }
    }
}
