use specs::{System, ReadStorage};

pub struct DrawIcon;

impl<'a> System<'a> for DrawIcon {
  type SystemData = ReadStorage<'a, Icon>;

  fn run(&mut self, icons: Self::SystemData) {
    use specs::Join;
    for icon in icons.join() {
      println!("Drawing {:?}", icon.ch);
    }
  }
}
