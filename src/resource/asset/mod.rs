/// Assets for game objects, to be serialized and deserialized from RON files
pub mod builder;
pub mod entity_template;
pub mod geography_template;
pub mod icons;
pub mod structure_template;

pub use builder::*;
pub use entity_template::*;
pub use geography_template::*;
pub use icons::*;
pub use structure_template::*;

use specs::{Component, VecStorage};
use std::collections::HashMap;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Assets {
    entities: HashMap<String, EntityTemplate>,
    icons: HashMap<String, Icon>,
    icons_by_ch: HashMap<char, String>,
    structures: HashMap<String, StructureTemplate>,
    geographies: HashMap<String, GeographyTemplate>,
    default_icon: Icon,
    pub ready: bool,
}

impl Default for Assets {
    fn default() -> Assets {
        Assets {
            entities: HashMap::new(),
            icons: HashMap::new(),
            icons_by_ch: HashMap::new(),
            structures: HashMap::new(),
            geographies: HashMap::new(),
            default_icon: Default::default(),
            ready: false,
        }
    }
}

impl Assets {
    pub fn add_icon(&mut self, name: &str, icon: Icon) {
        self.icons_by_ch.insert(icon.ch(), name.to_string());
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
    pub fn get_geographies(&self) -> &HashMap<String, GeographyTemplate> {
        &self.geographies
    }
    pub fn get_icon(&self, name: &str) -> &Icon {
        if let Some(icon) = self.icons.get(name) {
            &icon
        } else {
            &self.default_icon
        }
    }
    pub fn get_icon_by_ch(&self, ch: char) -> &Icon {
        if let Some(name) = self.icons_by_ch.get(&ch) {
            if let Some(icon) = self.icons.get(name) {
                return &icon;
            }
        }
        &self.default_icon
    }

    #[allow(unused)]
    pub fn get_geography(&self, name: &str) -> Option<&GeographyTemplate> {
        if let Some(template) = self.geographies.get(name) {
            Some(&template)
        } else {
            None
        }
    }

    pub fn get_structure(&self, name: &str) -> Option<&StructureTemplate> {
        if let Some(structure) = self.structures.get(name) {
            Some(&structure)
        } else {
            None
        }
    }

    pub fn process_geographies(&mut self) {
        let mut queue: Vec<(String, GeographyTemplate)> = Vec::new();
        let mut queue_processed: HashMap<String, GeographyTemplate> = HashMap::new();
        let mut marks: Vec<usize> = Vec::new();
        for (name, template) in self.geographies.drain() {
            queue.push((name, template));
        }
        let mut iterations = 0;
        while !queue.is_empty() {
            for (i, item) in queue.iter_mut().enumerate() {
                let (name, template) = item;
                if let Some(ref parent_name) = template.parent {
                    if let Some(parent) = queue_processed.get_mut(parent_name) {
                        println!("applying inheritance {} -> {}", parent_name, name);
                        template.inherit(parent);
                        marks.push(i);
                    }
                } else {
                    marks.push(i);
                }
            }
            while let Some(i) = marks.pop() {
                let (name, template) = queue.remove(i);
                queue_processed.insert(name, template);
            }
            iterations += 1;
            if iterations > 5 {
                panic!("too much nesting in geography templates (limit 5), or a possibly cyclic inheritance dependency");
            }
        }
        println!("finished inheritance for geographies");
        self.geographies = queue_processed;
    }
}
