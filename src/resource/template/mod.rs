/// Templates for game objects, to be serialized and deserialized from RON files
pub mod builder;
pub mod entity_template;
pub mod geography_template;
pub mod icon_template;
pub mod structure_template;

pub use builder::*;
pub use entity_template::*;
pub use geography_template::*;
pub use icon_template::*;
pub use structure_template::*;

pub trait Template {}

#[derive(Copy, Clone)]
pub enum TemplateType {
    Icon,
    Entity,
    Geography,
    Structure,
}

use specs::{Component, VecStorage};
use std::collections::HashMap;
#[derive(Component)]
#[storage(VecStorage)]
pub struct Templates {
    entities: HashMap<String, EntityTemplate>,
    icons: HashMap<String, IconTemplate>,
    structures: HashMap<String, StructureTemplate>,
    geographies: HashMap<String, GeographyTemplate>,
    pub ready: bool,
}

impl Default for Templates {
    fn default() -> Templates {
        Templates {
            entities: HashMap::new(),
            icons: HashMap::new(),
            structures: HashMap::new(),
            geographies: HashMap::new(),
            ready: false,
        }
    }
}

impl Templates {
    pub fn add_icon(&mut self, name: String, template: IconTemplate) {
        self.icons.insert(name, template);
    }
    pub fn add_entity(&mut self, name: String, template: EntityTemplate) {
        self.entities.insert(name, template);
    }
    pub fn add_geography(&mut self, name: String, template: GeographyTemplate) {
        self.geographies.insert(name, template);
    }
    pub fn add_structure(&mut self, name: String, template: StructureTemplate) {
        self.structures.insert(name, template);
    }
    pub fn len(&self) -> usize {
        self.entities.len() + self.icons.len() + self.structures.len() + self.geographies.len()
    }
    pub fn entity_len(&self) -> usize {
        self.entities.len()
    }
    pub fn icon_len(&self) -> usize {
        self.icons.len()
    }
    pub fn geography_len(&self) -> usize {
        self.geographies.len()
    }
    pub fn structure_len(&self) -> usize {
        self.structures.len()
    }
    /// chooses a random geography based on a random number <selector>
    pub fn choose_geography(&self, selector: f32) -> GeographyTemplate {
        let keys: Vec<String> = self.geographies.keys().map(|k| k.clone()).collect();
        let len = keys.len() as f32;
        self.geographies
            .get(keys.get((len * (selector % len)).floor() as usize).unwrap())
            .unwrap()
            .clone()
    }

    pub fn get_icon(&self, name: String) -> IconTemplate {
        println!("Looking up icon {}", name);
        self.icons
            .get(&name)
            .unwrap_or(&IconTemplate::default())
            .clone()
    }
}