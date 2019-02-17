use crate::component::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum GeographyTag {
    Forest,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GroundCover {
    pub frequency: f32,
    /// base colors for the ground cover
    pub colors: Colors,
    /// a string referring to an icon by file stem (icons/grass.ron = "grass" here)
    pub icon: IconRef,
}

/// stub
#[derive(Clone, Serialize, Deserialize)]
pub struct GeographyTemplate {
    #[serde(default)]
    /// tags that apply to this region, which will be used during object generation
    pub tags: Option<Vec<GeographyTag>>,
    #[serde(default)]
    /// list of structures that may appear in this map (density controlled elsewhere?)
    pub structures: Option<Vec<String>>,
    #[serde(default)]
    /// description used when viewing on (unimplemented) world map
    pub description: Option<Description>,
    #[serde(default)]
    /// a string referring to an icon by file stem, used on world map (icons/grass.ron = "grass" here)
    pub icon: Option<IconRef>,
    #[serde(default)]
    /// colors shown on world map for this geography type
    pub colors: Option<Colors>,
    #[serde(default)]
    /// base ground cover, blended according to frequency
    pub ground_cover: Option<Vec<GroundCover>>,
    #[serde(default)]
    /// scatter objects, placed independently according to frequency
    pub scatter: Option<Vec<GroundCover>>,
}

impl Default for GeographyTemplate {
    fn default() -> GeographyTemplate {
        GeographyTemplate {
            tags: None,
            structures: None,
            description: None,
            icon: None,
            colors: None,
            ground_cover: None,
            scatter: None,
        }
    }
}
