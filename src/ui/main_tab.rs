use super::{
    AttributeDetailWindow, Definition3dPanel, DefinitionDetailPanel, DefinitionSelectPanel,
    DefinitionsStore, State,
};
use egui::{CentralPanel, Id, ScrollArea, SidePanel, TopBottomPanel};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MainTab {
    definition_select_panel: DefinitionSelectPanel,
    definition_detail_panel: DefinitionDetailPanel,
    definition_3d_panel: Definition3dPanel,
    attribute_detail_windows: Vec<AttributeDetailWindow>,
    window_id: u32,
}

impl Default for MainTab {
    fn default() -> Self {
        let mut definition_select_panel = DefinitionSelectPanel::default();
        let definition_3d_panel =
            Definition3dPanel::new(None, definition_select_panel.selector_mut());

        Self {
            definition_select_panel,
            definition_detail_panel: DefinitionDetailPanel::default(),
            definition_3d_panel,
            attribute_detail_windows: Vec::new(),
            window_id: 0,
        }
    }
}

impl MainTab {
    pub fn creation_context(&mut self, cc: &eframe::CreationContext<'_>) {
        self.definition_3d_panel.creation_context(cc);
    }

    pub fn destory(&mut self, gl: Option<&eframe::glow::Context>) {
        self.definition_3d_panel.destroy(gl);
    }

    pub fn add_attribute_detail_window(&mut self, mut window: AttributeDetailWindow) {
        window.set_id(Id::new(format!("window_{}", self.window_id)));
        self.attribute_detail_windows.push(window);
        self.window_id += 1;
    }

    pub fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
        state: &mut State,
        definitions_store: &mut DefinitionsStore,
    ) {
        for window in &mut self.attribute_detail_windows {
            window.ui(
                ctx,
                state,
                definitions_store,
                self.definition_select_panel.selector_mut(),
            );
        }
        self.attribute_detail_windows.retain(|w| w.is_open());

        SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(80.0..=500.0)
            .show(ctx, |ui| {
                self.definition_select_panel.ui(ui, definitions_store);
            });

        SidePanel::right("right_panel")
            .resizable(true)
            .default_width(300.0)
            .width_range(80.0..=800.0)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(4.0);
                    self.definition_3d_panel
                        .ui(ui, self.definition_select_panel.selector_mut());
                    ui.add_space(4.0);
                });
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
    }
}
