use specs::{System, ReadStorage};

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
