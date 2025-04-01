use super::{
    definitions_store::DefinitionPointer, DefinitionMultiSelect, DefinitionSearch,
    DefinitionSelect, DefinitionSingleSelect, DefinitionsStore,
};
use egui::{Checkbox, Layout, RichText};
use egui_extras::{Size, StripBuilder};
use std::{cell::RefCell, rc::Rc};

pub struct DefinitionSelectPanel {
    search: Rc<RefCell<DefinitionSearch>>,
    selector: Rc<RefCell<DefinitionSingleSelect>>,
    selector_observer_id: u32,
    multi_selector: Option<Rc<RefCell<DefinitionMultiSelect>>>,
    auto_select: bool,
}

impl Default for DefinitionSelectPanel {
    fn default() -> Self {
        Self::single_select()
    }
}

impl DefinitionSelectPanel {
    pub fn single_select() -> Self {
        let mut selector = DefinitionSingleSelect::default();
        let selector_observer_id = selector.register_observer();

        Self {
            search: Default::default(),
            selector: Rc::new(RefCell::new(selector)),
            selector_observer_id,
            multi_selector: None,
            auto_select: false,
        }
    }

    pub fn multi_select() -> Self {
        let multi_selector = DefinitionMultiSelect::default();

        Self {
            multi_selector: Some(Rc::new(RefCell::new(multi_selector))),
            auto_select: true,
            ..Self::single_select()
        }
    }

    pub fn use_search(&mut self, search: Rc<RefCell<DefinitionSearch>>) {
        self.search = search;
    }

    pub fn use_selector(&mut self, selector: Rc<RefCell<DefinitionSingleSelect>>) {
        self.selector_observer_id = selector.borrow_mut().register_observer();
        self.selector = selector;
    }

    pub fn register_observer(&mut self) -> u32 {
        self.selector.borrow_mut().register_observer()
    }

    pub fn selector(&self) -> Rc<RefCell<DefinitionSingleSelect>> {
        self.selector.clone()
    }

    pub fn selected_definition(&self) -> Option<DefinitionPointer> {
        self.selector.borrow().selected()
    }

    pub fn check_update(&mut self, observer_id: u32) -> Option<bool> {
        self.selector.borrow_mut().check_update(observer_id)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, definitions_store: &mut DefinitionsStore) {
        ui.add_space(6.0);

        self.search.borrow_mut().ui(ui);

        {
            let items = self.search.borrow().get_result(definitions_store);
            self.ui_select_all(ui, &items);
            ui.add_space(6.0);
            self.ui_list(ui, &items);
        }

        self.search.borrow_mut().update_search(definitions_store);

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

    fn ui_select_all(&self, ui: &mut egui::Ui, items: &[(String, DefinitionPointer)]) {
        if let Some(multi_select) = &self.multi_selector {
            let count = multi_select.borrow().count();
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
                    multi_select
                        .borrow_mut()
                        .set_selection(items.iter().map(|(_, ptr)| ptr));
                } else {
                    multi_select.borrow_mut().clear();
                }
            }
        }
    }

    fn ui_list(&self, ui: &mut egui::Ui, items: &[(String, DefinitionPointer)]) {
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
}

pub struct DefinitionMultiSelectPanel {
    panel: DefinitionSelectPanel,
}

impl Default for DefinitionMultiSelectPanel {
    fn default() -> Self {
        Self {
            panel: DefinitionSelectPanel::multi_select(),
        }
    }
}

impl DefinitionMultiSelectPanel {
    pub fn use_selector(&mut self, selector: Rc<RefCell<DefinitionSingleSelect>>) {
        self.panel.use_selector(selector);
    }

    pub fn use_search(&mut self, search: Rc<RefCell<DefinitionSearch>>) {
        self.panel.use_search(search);
    }

    pub fn register_observer(&mut self) -> u32 {
        self.panel.register_observer()
    }

    pub fn selector(&self) -> Rc<RefCell<DefinitionSingleSelect>> {
        self.panel.selector()
    }

    pub fn multi_selector(&self) -> Rc<RefCell<DefinitionMultiSelect>> {
        self.panel.multi_selector.as_ref().unwrap().clone()
    }

    pub fn selected_definition(&self) -> Option<DefinitionPointer> {
        self.panel.selected_definition()
    }

    pub fn check_update(&mut self, observer_id: u32) -> Option<bool> {
        self.panel.check_update(observer_id)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, definitions_store: &mut DefinitionsStore) {
        self.panel.ui(ui, definitions_store);
    }
}
