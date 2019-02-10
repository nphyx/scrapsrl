/// Templates for game objects, to be serialized and deserialized from RON files
use specs::{World, Component, VecStorage};
use serde::Serialize;
use std::collections::HashMap;

pub mod builder;

pub use builder::*;

use crate::component::*;

#[derive(Clone,Serialize)]
pub struct EntityTemplate {
  brain: Option<AIBrain>,
  character: Option<Character>,
  colors: Option<Colors>,
  description: Option<Description>,
  icon: Option<Icon>,
  notification: Option<NotificationInteraction>,
  solid: Option<Solid>
}

impl Default for EntityTemplate {
  fn default() -> EntityTemplate {
    EntityTemplate{
      brain: None,
      character: None,
      colors: None,
      description: None,
      icon: None,
      notification: None,
      solid: None
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
    match &self.brain {
      Some(brain) => { builder = builder.with(brain.clone()); }
      _ => {}
    }
    match &self.character {
      Some(character) => { builder = builder.with(character.clone()); }
      _ => {}
    }
    match &self.colors {
      Some(colors) => { builder = builder.with(colors.clone()); }
      _ => {}
    }
    match &self.description {
      Some(description) => { builder = builder.with(description.clone()); }
      _ => {}
    }
    match &self.icon {
      Some(icon) => { builder = builder.with(icon.clone()); }
      _ => {}
    }
    match &self.notification {
      Some(notification) => { builder = builder.with(notification.clone()); }
      _ => {}
    }
    match &self.solid {
      Some(_) => { builder = builder.with(Solid); }
      _ => {}
    }
    builder
  }
}

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
