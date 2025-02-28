mod definition;
pub use definition::{SwBlockDefinition, SwBlockDefinitionMeshKey, SwBlockDefinitionMeshes};
pub mod definition_schema;
mod surface_mesh;
mod sw_mesh;
pub use surface_mesh::create_surface_object;
mod definition_attribute;
pub use definition_attribute::{DefinitionAttribute, DefinitionAttributeValue};
