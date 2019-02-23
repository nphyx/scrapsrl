/// Assets for game objects, to be serialized and deserialized from RON files
pub mod builder;
pub mod entity_template;
pub mod geography_template;
pub mod icon;
pub mod structure_template;

pub use builder::*;
pub use entity_template::*;
pub use geography_template::*;
pub use icon::*;
pub use structure_template::*;

use specs::{Component, VecStorage};
use std::collections::HashMap;

use crate::component::Region;
use crate::resource::WorldState;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Assets {
    entities: HashMap<String, EntityTemplate>,
    icons: HashMap<String, Icon>,
    structures: HashMap<String, StructureTemplate>,
    geographies: HashMap<String, GeographyTemplate>,
    pub ready: bool,
}

impl Default for Assets {
    fn default() -> Assets {
        Assets {
            entities: HashMap::new(),
            icons: HashMap::new(),
            structures: HashMap::new(),
            geographies: HashMap::new(),
            ready: false,
        }
    }
}

impl Assets {
    pub fn add_icon(&mut self, name: &str, icon: Icon) {
        self.icons.insert(name.to_string(), icon);
    }
    pub fn add_entity(&mut self, name: &str, template: EntityTemplate) {
        self.entities.insert(name.to_string(), template);
    }
    pub fn add_geography(&mut self, name: &str, template: GeographyTemplate) {
        self.geographies.insert(name.to_string(), template);
    }
    pub fn add_structure(&mut self, name: &str, template: StructureTemplate) {
        self.structures.insert(name.to_string(), template);
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
    pub fn choose_geography(
        &self,
        sample: f32,
        region: Region,
        world: &WorldState,
    ) -> &GeographyTemplate {
        let pop = world.get_pop(region);
        // let keys: Vec<String> = self.geographies.keys().map(|k| k.clone()).collect();
        let choices: Vec<&GeographyTemplate> = self
            .geographies
            .values()
            .filter(|item| item.population_range[0] < pop && item.population_range[1] > pop)
            .collect();
        let len = choices.len() as f32;
        let choice = *choices
            .get((len * (sample % len)).floor() as usize)
            .expect("no available geographies matching the given tag");
        println!("chose {:?}", choice.tags);
        choice
    }

    pub fn get_icon(&self, name: &str) -> Icon {
        if let Some(icon) = self.icons.get(name) {
            icon.clone()
        } else {
            Icon::default()
        }
    }
}
