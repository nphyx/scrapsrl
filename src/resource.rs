#[derive(Default,Clone)]
pub struct MapGenRequested(pub bool);

#[derive(Default,Clone)]
pub struct MapWidth(pub i32);

#[derive(Default,Clone)]
pub struct MapHeight(pub i32);

#[derive(Default,Clone)]
pub struct WindowClosed(pub bool);
