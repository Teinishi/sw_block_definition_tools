use super::{
    tab::Tab, utils::ui_center, AppAction, AttributeDetailWindow, Definition3dPanel,
    DefinitionDetailPanel, DefinitionSearch, DefinitionSelect, DefinitionSelectPanel,
    DefinitionSingleSelect, DefinitionsStore, State,
};
use egui::{Button, CentralPanel, Frame, Id, ScrollArea, SidePanel, TopBottomPanel};
use std::{cell::RefCell, rc::Rc};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MainTab {
    #[serde(skip)]
    definition_select_panel: DefinitionSelectPanel,
    #[serde(skip)]
    selector_observer_id: u32,

    definition_detail_panel: DefinitionDetailPanel,
    definition_3d_panel: Definition3dPanel,
    attribute_detail_windows: Vec<AttributeDetailWindow>,
    window_id: u32,
}

impl Default for MainTab {
    fn default() -> Self {
        let mut definition_select_panel = DefinitionSelectPanel::default();
        let selector_observer_id = definition_select_panel.register_observer();
        let definition_3d_panel = Definition3dPanel::new(None);

        Self {
            definition_select_panel,
            selector_observer_id,

            definition_detail_panel: DefinitionDetailPanel,
            definition_3d_panel,
            attribute_detail_windows: Vec::new(),
            window_id: 0,
        }
    }
}

impl Tab for MainTab {
    fn creation_context(&mut self, cc: &eframe::CreationContext<'_>) {
        self.definition_3d_panel.creation_context(cc);
    }

    fn use_selector(&mut self, selector: std::rc::Rc<std::cell::RefCell<DefinitionSingleSelect>>) {
        self.selector_observer_id = selector.borrow_mut().register_observer();
        self.definition_select_panel.use_selector(selector);
    }

    fn use_search(&mut self, search: Rc<RefCell<DefinitionSearch>>) {
        self.definition_select_panel.use_search(search);
    }

    fn destroy(&mut self, gl: Option<&eframe::glow::Context>) {
        self.definition_3d_panel.destroy(gl);
    }

    fn reset(&mut self) {
        self.definition_select_panel = Default::default();
        self.selector_observer_id = self.definition_select_panel.register_observer();
        self.definition_detail_panel = Default::default();
        self.definition_3d_panel.reset();
        self.attribute_detail_windows = Vec::new();
        self.window_id = 0;
    }

    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
        state: &mut State,
        definitions_store: &mut DefinitionsStore,
    ) -> Option<AppAction> {
        for window in &mut self.attribute_detail_windows {
            window.ui(
                ctx,
                state,
                definitions_store,
                self.definition_select_panel.selector(),
            );
        }
        self.attribute_detail_windows.retain(|w| w.is_open());

        if state.rom_path.is_none() {
            let mut action = None;

            CentralPanel::default().show(ctx, |ui| {
                let size = egui::vec2(200.0, 60.0);
                ui_center(ui, size, |ui| {
                    if ui.add_sized(size, Button::new("Open rom folder")).clicked() {
                        action = Some(AppAction::SelectRomFolder);
                    }
                });
            });

            return action;
        }

        SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(80.0..=500.0)
            .show(ctx, |ui| {
                self.definition_select_panel.ui(ui, definitions_store);
            });

        SidePanel::right("right_panel")
            .frame(Frame::side_top_panel(&ctx.style()).inner_margin(0.0))
            .resizable(true)
            .default_width(300.0)
            .width_range(80.0..=800.0)
            .show(ctx, |ui| {
                self.definition_3d_panel.ui(
                    ui,
                    definitions_store,
                    self.definition_select_panel.selected_definition(),
                    self.definition_select_panel
                        .check_update(self.selector_observer_id)
                        .unwrap_or(false),
                );
            });

        TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);

                ui.checkbox(&mut state.show_all, "Show all");
                ui.checkbox(&mut state.hide_default, "Hide zero/empty");

                ui.add_space(4.0);
            });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().show(ui, |ui| {
                ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                if let Some(definition) = self.definition_select_panel.selected_definition() {
                    let new_window = self.definition_detail_panel.ui(ui, state, definition);
                    if let Some(w) = new_window {
                        self.add_attribute_detail_window(w);
                    }
                    ui.add_space(10.0);
                }
            });
        });

        None
    }
}

impl MainTab {
    pub fn add_attribute_detail_window(&mut self, mut window: AttributeDetailWindow) {
        window.set_id(Id::new(format!("window_{}", self.window_id)));
        self.attribute_detail_windows.push(window);
        self.window_id += 1;
    }
}
