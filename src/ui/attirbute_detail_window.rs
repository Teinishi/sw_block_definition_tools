#[derive(serde::Serialize, serde::Deserialize)]
pub struct AttributeDetailWindow {
    open: bool,
    id: Option<egui::Id>,
    specifier: AttributeSpecifier,
}

impl AttributeDetailWindow {
    pub fn new(specifier: AttributeSpecifier) -> Self {
        Self {
            open: true,
            id: None,
            specifier,
        }
    }

    pub fn definition_attribute(name: String) -> Self {
        Self::new(AttributeSpecifier::DefinitionAttribute(name))
    }

    pub fn set_id(&mut self, id: egui::Id) {
        self.id = Some(id);
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if let Some(id) = self.id {
            egui::Window::new(self.specifier.to_string())
                .id(id)
                .open(&mut self.open)
                .show(ctx, |_ui| {
                    // TODO
                });
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum AttributeSpecifier {
    DefinitionAttribute(String),
}

impl std::fmt::Display for AttributeSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DefinitionAttribute(name) => {
                write!(f, "{} in <definition>", name)
            }
        }
    }
}
