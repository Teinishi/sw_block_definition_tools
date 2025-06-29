use super::{BlockDefinition, DefinitionRegistory};
use crate::sw_block_definition::{
    AttributeSpecifier, AttributeValue, GetAttributeValueRoot, IsDefault,
};
use std::collections::{BTreeMap, BTreeSet};

pub type AttributeDefinitionMap = BTreeMap<String, (BlockDefinition, BTreeSet<AttributeValue>)>;
pub type AttributeValueMap = BTreeMap<AttributeValue, BTreeMap<String, BlockDefinition>>;

#[derive(Debug)]
pub struct AttributeValueContainer {
    values: Vec<(BlockDefinition, AttributeValue)>,
}

impl AttributeValueContainer {
    pub fn new(
        registory: &DefinitionRegistory,
        specifier: &AttributeSpecifier,
        hide_defalt: bool,
    ) -> Self {
        let mut values = Vec::new();

        for (_, _, definition) in registory.definitions() {
            definition.use_data(|data| {
                for value in specifier.get_value_root(data) {
                    if !hide_defalt || !value.is_default() {
                        values.push((definition.clone(), value));
                    }
                }
            });
        }

        Self { values }
    }

    // ファイル名と値の集合のペア
    pub fn definition_map(&self) -> AttributeDefinitionMap {
        let mut map: AttributeDefinitionMap = BTreeMap::new();
        for (definition, value) in &self.values {
            let filename = definition.filename();
            if let Some(entry) = map.get_mut(filename) {
                entry.1.insert(value.clone());
            } else {
                map.insert(
                    filename.to_string(),
                    (definition.clone(), BTreeSet::from([value.clone()])),
                );
            }
        }
        map
    }

    // 値とファイル名の集合のペア
    pub fn value_map(&self) -> AttributeValueMap {
        let mut map: AttributeValueMap = BTreeMap::new();
        for (definition, value) in &self.values {
            let filename = definition.filename();
            if let Some(entries) = map.get_mut(value) {
                entries.insert(filename.to_string(), definition.clone());
            } else {
                let mut entries = BTreeMap::new();
                entries.insert(filename.to_string(), definition.clone());
                map.insert(value.clone(), entries);
            }
        }
        map
    }
}
