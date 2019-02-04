use specs::{Component, VecStorage};

pub struct Icon {
  pub ch: char
}

impl Component for Icon {
  type Storage = VecStorage<Self>;
}
