use specs::{Component, VecStorage};

pub struct Description { 
  pub short: String,
  pub long: String
}

impl Component for Description {
  type Storage = VecStorage<Self>;
}
