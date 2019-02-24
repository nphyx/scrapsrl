use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StructureConnectionType {
    Road,
    Structure(StructureTemplate),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StructureConnectionMethod {
    Driveway,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructureConnection {
    connection_type: StructureConnectionType,
    connection_method: StructureConnectionMethod,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructureTemplate {
    pub min_width: i32,
    pub max_width: i32,
    pub min_height: i32,
    pub max_height: i32,
    /// perimeter is *inside* the bounds, so account for it in min/max properties
    pub perimeter: i32,
    /// a special instruction for connecting to roads, other structures, etc
    pub connect_to: Option<Vec<StructureConnection>>,
}

impl Default for StructureTemplate {
    fn default() -> StructureTemplate {
        StructureTemplate {
            min_width: 3,
            max_width: 3,
            min_height: 3,
            max_height: 3,
            perimeter: 1,
            connect_to: None,
        }
    }
}
