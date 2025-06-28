use crate::{
    definition_hub::DefinitionRegistory,
    state::State,
    ui::{app::BlockSingleSelection, components::SharedDefinitionSearch, AppAction},
};

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
    fn creation_context(
        &mut self,
        cc: &eframe::CreationContext<'_>,
        search: SharedDefinitionSearch,
        selection: BlockSingleSelection,
    ) {
    }

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
        registory: &mut DefinitionRegistory,
    ) -> Option<AppAction>;
}
