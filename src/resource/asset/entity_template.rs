use serde::{Deserialize, Serialize};
use specs::World;

use crate::component::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct EntityTemplate {
    brain: Option<AIBrain>,
    character: Option<Character>,
    colors: Option<Colors>,
    description: Option<Description>,
    icon: Option<IconRef>,
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
#[allow(unused)]
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

#[allow(unused)]
pub struct EntityTemplateBuilder {
    template: EntityTemplate,
}

#[allow(unused)]
impl EntityTemplateBuilder {
    pub fn new() -> EntityTemplateBuilder {
        EntityTemplateBuilder {
            template: EntityTemplate::default(),
        }
    }

    pub fn from(template: EntityTemplate) -> EntityTemplateBuilder {
        EntityTemplateBuilder { template }
    }

    pub fn brain(&mut self) -> &mut EntityTemplateBuilder {
        self.template.brain = Some(AIBrain::default());
        self
    }

    pub fn character(&mut self, character: Character) -> &mut EntityTemplateBuilder {
        self.template.character = Some(character);
        self
    }

    pub fn colors(&mut self, fg: Color, bg: Color) -> &mut EntityTemplateBuilder {
        self.template.colors = Some(Colors { fg, bg });
        self
    }

    pub fn solid(&mut self) -> &mut EntityTemplateBuilder {
        self.template.solid = Some(Solid);
        self
    }

    pub fn description(&mut self, short: &str, long: &str) -> &mut EntityTemplateBuilder {
        self.template.description = Some(Description {
            short: short.to_string(),
            long: long.to_string(),
        });
        self
    }

    pub fn icon(&mut self, name: String) -> &mut EntityTemplateBuilder {
        self.template.icon = Some(IconRef { name });
        self
    }

    pub fn notification(&mut self, header: String, body: String) -> &mut EntityTemplateBuilder {
        self.template.notification = Some(NotificationInteraction { header, body });
        self
    }

    pub fn build(&mut self) -> EntityTemplate {
        self.template.clone()
    }
}
