use specs::{Component, VecStorage};

#[derive(Clone,Component,Debug,Default)]
#[storage(VecStorage)]
struct NotificationBody {
  header: String,
  body: String
}
