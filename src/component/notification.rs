use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct NotificationBody {
  header: String,
  body: String
}
