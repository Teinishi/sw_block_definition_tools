use super::{DefinitionSingleSelect, DefinitionsStore, State};

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

pub trait Tab {
    #[allow(unused_variables)]
    fn creation_context(&mut self, cc: &eframe::CreationContext<'_>) {}

    #[allow(unused_variables)]
    fn use_selector(&mut self, selector: std::rc::Rc<std::cell::RefCell<DefinitionSingleSelect>>) {}

    #[allow(unused_variables)]
    fn destroy(&mut self, gl: Option<&eframe::glow::Context>) {}

    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
        state: &mut State,
        definitions_store: &mut DefinitionsStore,
    );
}
