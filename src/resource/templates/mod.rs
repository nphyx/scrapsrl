use serde::Serialize;
/// Templates for game objects, to be serialized and deserialized from RON files
use specs::World;

pub mod builder;

pub use builder::*;

use crate::component::*;

#[derive(Clone, Serialize)]
pub struct EntityTemplate {
    brain: Option<AIBrain>,
    character: Option<Character>,
    colors: Option<Colors>,
    description: Option<Description>,
    icon: Option<Icon>,
    notification: Option<NotificationInteraction>,
    solid: Option<Solid>,
}

impl Default for EntityTemplate {
    fn default() -> EntityTemplate {
        EntityTemplate {
            brain: None,
            character: None,
            colors: None,
            description: None,
            icon: None,
            notification: None,
            solid: None,
        }
    }
}

use specs::Builder;
impl EntityTemplate {
    pub fn create() -> EntityTemplateBuilder {
        EntityTemplateBuilder::new()
    }

    pub fn to_world<'a>(&self, world: &'a mut World) -> impl Builder + 'a {
        let mut builder = world.create_entity();
        if let Some(brain) = &self.brain {
            builder = builder.with(brain.clone());
        }
        if let Some(character) = &self.character {
            builder = builder.with(character.clone());
        }
        if let Some(colors) = &self.colors {
            builder = builder.with(colors.clone());
        }
        if let Some(description) = &self.description {
            builder = builder.with(description.clone());
        }
        if let Some(icon) = &self.icon {
            builder = builder.with(icon.clone());
        }
        if let Some(notification) = &self.notification {
            builder = builder.with(notification.clone());
        }
        if self.solid.is_some() {
            builder = builder.with(Solid);
        }
        builder
    }
}

/* FIXME unused
use specs::{Component, VecStorage};
use std::collections::HashMap;
#[derive(Component)]
#[storage(VecStorage)]
pub struct Templates {
  contents: HashMap<String, EntityTemplate>
}

impl Default for Templates {
  fn default() -> Templates {
    Templates{contents: HashMap::new()}
  }
}

impl Templates {
  fn add(&mut self, name: String, template: EntityTemplate) {
    self.contents.insert(name, template);
  }
}
*/
