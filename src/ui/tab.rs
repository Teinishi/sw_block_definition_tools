use std::{cell::RefCell, rc::Rc};

use super::{AppAction, DefinitionSingleSelect, DefinitionsStore, State};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub enum TabVariants {
    Main,
    SaveImage,
    Settings,
}

impl Default for TabVariants {
    fn default() -> Self {
        Self::Main
    }
}

pub trait Tab: Default {
    #[allow(unused_variables)]
    fn creation_context(&mut self, cc: &eframe::CreationContext<'_>) {}

    #[allow(unused_variables)]
    fn use_selector(&mut self, selector: Rc<RefCell<DefinitionSingleSelect>>) {}

    #[allow(unused_variables)]
    fn use_search_text(&mut self, search_text: Rc<RefCell<String>>) {}

    #[allow(unused_variables)]
    fn destroy(&mut self, gl: Option<&eframe::glow::Context>) {}

    #[allow(unused_variables)]
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
        state: &mut State,
        definitions_store: &mut DefinitionsStore,
    ) -> Option<AppAction>;
}
