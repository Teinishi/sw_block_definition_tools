use super::tab::Tab;
use crate::{
    definition_hub::DefinitionRegistory,
    state::State,
    ui::{
        app::BlockSingleSelection,
        components::SharedDefinitionSearch,
        panels::{Definition3dPanel, DefinitionDetailPanel, DefinitionSelectPanel},
        utils::ui_center,
        windows::AttributeDetailWindow,
        AppAction,
    },
};
use egui::{Button, CentralPanel, Frame, Id, ScrollArea, SidePanel, TopBottomPanel};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MainTab {
    #[serde(skip)]
    definition_select_panel: DefinitionSelectPanel,
    #[serde(skip)]
    selection: BlockSingleSelection,
    definition_detail_panel: DefinitionDetailPanel,
    definition_3d_panel: Definition3dPanel,
    attribute_detail_windows: Vec<AttributeDetailWindow>,
    window_id: u32,
}

impl Default for MainTab {
    fn default() -> Self {
        let definition_select_panel = DefinitionSelectPanel::default();
        let selection = definition_select_panel.selection().clone();
        let definition_3d_panel = Definition3dPanel::new(None);

        Self {
            definition_select_panel,
            selection,
            definition_detail_panel: DefinitionDetailPanel,
            definition_3d_panel,
            attribute_detail_windows: Vec::new(),
            window_id: 0,
        }
    }
}

impl Tab for MainTab {
    fn creation_context(
        &mut self,
        cc: &eframe::CreationContext<'_>,
        search: SharedDefinitionSearch,
        selection: BlockSingleSelection,
    ) {
        self.definition_3d_panel.creation_context(cc);
        self.definition_select_panel.use_search(search);
        self.definition_select_panel
            .use_selection(selection.clone());
        self.selection = selection;
    }

    fn destroy(&mut self, gl: Option<&eframe::glow::Context>) {
        self.definition_3d_panel.destroy(gl);
    }

    fn reset(&mut self) {
        self.definition_select_panel = Default::default();
        self.selection = self.definition_select_panel.selection().clone();
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
        registory: &mut DefinitionRegistory,
    ) -> Option<AppAction> {
        for window in &mut self.attribute_detail_windows {
            window.ui(
                ctx,
                state,
                registory,
                self.definition_select_panel.selection(),
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
                self.definition_select_panel.ui(ui, registory);
            });

        let definition = self
            .definition_select_panel
            .selection()
            .get()
            .and_then(|key| registory.get(&key))
            .cloned();
        let selection_changed = self.selection.check_update();

        SidePanel::right("right_panel")
            .frame(Frame::side_top_panel(&ctx.style()).inner_margin(0.0))
            .resizable(true)
            .default_width(300.0)
            .width_range(80.0..=800.0)
            .show(ctx, |ui| {
                self.definition_3d_panel.ui(
                    ui,
                    state,
                    registory,
                    definition.as_ref(),
                    selection_changed,
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
                let new_window = self.definition_detail_panel.ui(
                    ui,
                    state,
                    registory,
                    self.definition_select_panel.selection(),
                );
                if let Some(w) = new_window {
                    self.add_attribute_detail_window(w);
                }
                ui.add_space(10.0);
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
