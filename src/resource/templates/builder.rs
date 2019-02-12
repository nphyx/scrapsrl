use super::EntityTemplate;
use crate::component::*;

pub struct EntityTemplateBuilder {
    template: EntityTemplate,
}

impl EntityTemplateBuilder {
    pub fn new() -> EntityTemplateBuilder {
        EntityTemplateBuilder {
            template: EntityTemplate::default(),
        }
    }

    /* FIXME unused impl From trait
    pub fn from<'a>(template: EntityTemplate) -> EntityTemplateBuilder {
      EntityTemplateBuilder{template}
    }
    */

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

    pub fn icon(&mut self, ch: char) -> &mut EntityTemplateBuilder {
        self.template.icon = Some(Icon { ch });
        self
    }

    /* FIXME unused will use when porting interact objects
    pub fn notification(&mut self, header: String, body: String) -> &mut EntityTemplateBuilder {
      self.template.notification = Some(NotificationInteraction{header, body});
      self
    }
    */

    pub fn build(&mut self) -> EntityTemplate {
        self.template.clone()
    }
}
