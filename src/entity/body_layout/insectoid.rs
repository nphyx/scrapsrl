use crate::entity::entity_part::{
  EntityComponent as Component,
  EntityComponentLocation as Location,
  EntityComponentVariety as Variety,
  EntityComponentSide as Side
};

pub fn insectoid_head() -> Component {
  let mut head = Component::new(Variety::Head, Side::Front);
  head.attach(Location::On, Component::new(Variety::Eye, Side::Left));
  head.attach(Location::On, Component::new(Variety::Eye, Side::Right));
  head.attach(Location::On, Component::new(Variety::Antenna, Side::Left));
  head.attach(Location::On, Component::new(Variety::Antenna, Side::Right));
  head.attach(Location::On, Component::new(Variety::Mandible, Side::NoSide));
  head
}

pub fn insectoid_leg(side: Side) -> Component {
  Component::new(Variety::Leg, side)
}

pub fn insectoid() -> Component {
  let mut body = Component::new(Variety::Thorax, Side::NoSide);
  body.attach(Location::On, insectoid_head());
  body.attach(Location::Side, insectoid_leg(Side::Left));
  body.attach(Location::Side, insectoid_leg(Side::Left));
  body.attach(Location::Side, insectoid_leg(Side::Left));
  body.attach(Location::Side, insectoid_leg(Side::Right));
  body.attach(Location::Side, insectoid_leg(Side::Right));
  body.attach(Location::Side, insectoid_leg(Side::Right));
  body.attach(Location::On, Component::new(Variety::Abdomen, Side::Behind));
  body
}
