use std::collections::HashMap;

mod entity_part_location;
mod entity_part_side;
mod entity_part_variety;
pub use self::entity_part_side::EntityComponentSide;
pub use self::entity_part_location::EntityComponentLocation;
pub use self::entity_part_variety::EntityComponentVariety;

type EntityComponentChildren = HashMap<EntityComponentLocation, Vec<Box<EntityComponent>>>;

pub struct EntityComponent {
  pub variety: EntityComponentVariety,
  pub side: EntityComponentSide,
  pub children: EntityComponentChildren,
}

impl EntityComponent {

  pub fn new(
    variety: EntityComponentVariety, 
    side: EntityComponentSide) -> EntityComponent {
    EntityComponent{
      variety,
      side,
      children: EntityComponentChildren::new()
    }
  }

  pub fn attach(
    &mut self,
    location: EntityComponentLocation,
    component: EntityComponent) {
    let loc = self.children.get_mut(&location);
    match loc {
      Some(child) => { child.push(Box::new(component)); },
      None => { self.children.insert(location, vec![Box::new(component)]); }
    }
  }

  pub fn to_string(&self) -> String {
    format!("a {} {}", self.side.to_string(), self.variety.to_string())
  }
}
