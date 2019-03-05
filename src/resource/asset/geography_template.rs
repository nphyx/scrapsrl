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
    #[serde(default)]
    /// a short description to use for tiles of this type
    pub short: String,
    #[serde(default)]
    /// a long description to use for tiles of this type
    pub long: String,
}

impl Default for GroundCover {
    fn default() -> GroundCover {
        GroundCover {
            frequency: 1.0,
            colors: Colors {
                fg: Color::new(0, 0, 0),
                bg: Color::new(0, 0, 0),
            },
            icon: IconRef::default(),
            short: "perfectly generic ground".to_string(),
            long: "Someone forgot to create a tile for this bit of ground.".to_string(),
        }
    }
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
    /// will adopt all settings from this template if it is provided, overriding
    /// where this template has its own settings and incorporating all items from both
    /// in the case of vecs
    pub parent: Option<String>,
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
            parent: None,
        }
    }
}

impl GeographyTemplate {
    pub fn inherit(&mut self, parent: &mut GeographyTemplate) {
        if let Some(ref parent_tags) = parent.tags {
            if let Some(ref mut tags) = self.tags {
                for tag in parent_tags.iter() {
                    tags.push(tag.clone());
                }
            } else {
                self.tags = Some(parent_tags.clone());
            }
        }
        if let Some(ref parent_structures) = parent.structures {
            if let Some(ref mut structures) = self.structures {
                for structure in parent_structures.iter() {
                    structures.push(structure.clone());
                }
            } else {
                self.structures = Some(parent_structures.clone());
            }
        }
        if let Some(ref parent_description) = parent.description {
            if let None = self.description {
                self.description = Some(parent_description.clone());
            }
        }
        if let Some(ref parent_icon) = parent.icon {
            if let None = self.icon {
                self.icon = Some(parent_icon.clone());
            }
        }
        if let Some(ref parent_colors) = parent.colors {
            if let None = self.colors {
                self.colors = Some(parent_colors.clone());
            }
        }
        if let Some(ref parent_ground_cover) = parent.ground_cover {
            if let Some(ref mut ground_cover) = self.ground_cover {
                for structure in parent_ground_cover.iter() {
                    ground_cover.push(structure.clone());
                }
            } else {
                self.ground_cover = Some(parent_ground_cover.clone());
            }
        }
        if let Some(ref parent_scatter) = parent.scatter {
            if let Some(ref mut scatter) = self.scatter {
                for structure in parent_scatter.iter() {
                    scatter.push(structure.clone());
                }
            } else {
                self.scatter = Some(parent_scatter.clone());
            }
        }
        // all done, remove the parent so this doesn't get repeated
        // and improperly duplicated
        self.parent = None;
    }
}
