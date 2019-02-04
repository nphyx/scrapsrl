use specs::{System, ReadStorage};
use crate::component::{Icon, Description};

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

pub struct Describe;

impl<'a> System<'a> for Describe {
  type SystemData = ReadStorage<'a, Description>;

  fn run(&mut self, descriptions: Self::SystemData) {
    use specs::Join;
    for desc in descriptions.join() {
      println!("It's a {:?}", desc.short);
    }
  }
}
