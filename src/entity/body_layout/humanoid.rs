use crate::entity::entity_part::{
  EntityComponent as Component,
  EntityComponentLocation as Location,
  EntityComponentVariety as Variety,
  EntityComponentSide as Side
};

pub fn humanoid_head() -> Component {
  let mut head = Component::new(Variety::Head, Side::NoSide);
  head.attach(Location::On, Component::new(Variety::Eye, Side::Left));
  head.attach(Location::On, Component::new(Variety::Eye, Side::Right));
  head.attach(Location::On, Component::new(Variety::Ear, Side::Left));
  head.attach(Location::On, Component::new(Variety::Ear, Side::Right));
  head.attach(Location::On, Component::new(Variety::Nose, Side::NoSide));
  head.attach(Location::On, Component::new(Variety::Mouth, Side::NoSide));
  head
}

pub fn humanoid_arm(side: Side) -> Component {
  let mut arm = Component::new(Variety::Arm, side);
  arm.attach(Location::On, Component::new(Variety::Hand, Side::NoSide));
  arm
}

pub fn humanoid_leg(side: Side) -> Component {
  let mut leg = Component::new(Variety::Leg, side);
  leg.attach(Location::On, Component::new(Variety::Foot, Side::NoSide));
  leg
}

pub fn humanoid_hips() -> Component {
  let mut hips = Component::new(Variety::Hips, Side::NoSide);
  hips.attach(Location::Side, humanoid_leg(Side::Left));
  hips.attach(Location::Side, humanoid_leg(Side::Right));
  hips
}

pub fn humanoid() -> Component {
  let mut body = Component::new(Variety::Torso, Side::NoSide);

  body.attach(Location::Above, humanoid_head());
  body.attach(Location::Side, humanoid_arm(Side::Left));
  body.attach(Location::Side, humanoid_arm(Side::Right));
  body.attach(Location::Under, humanoid_hips());

  body
}
