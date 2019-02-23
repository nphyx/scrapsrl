use crate::component::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum GeographyTag {
    Forest,
    Rural,
    Urban,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
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
    /// lower and upper threshold of population levels in which the geography can occur
    /// from 0.0 - 1.0
    pub population_range: [f32; 2],
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
            population_range: [0.0, 1.0],
            structures: None,
            description: None,
            icon: None,
            colors: None,
            ground_cover: None,
            scatter: None,
        }
    }
}
