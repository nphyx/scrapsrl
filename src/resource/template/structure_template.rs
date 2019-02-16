use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StructureTemplate {
    min_width: u32,
    max_width: u32,
    min_height: u32,
    max_height: u32,
}
