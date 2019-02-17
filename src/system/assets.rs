use specs::{System, Write};
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use std::path::Path;

use crate::constants::{ICON_DIR, TEMPLATE_DIR};
use crate::resource::{
    Assets, EntityTemplate, GameStage, GameState, GeographyTemplate, Icon, StructureTemplate,
};

fn type_dir<'a>(template_type: AssetType) -> String {
    match template_type {
        AssetType::Icon => format!("{}/{}", ICON_DIR, "icons"),
        AssetType::Entity => format!("{}/{}", TEMPLATE_DIR, "entities"),
        AssetType::Geography => format!("{}/{}", TEMPLATE_DIR, "geographies"),
        AssetType::Structure => format!("{}/{}", TEMPLATE_DIR, "structures"),
    }
}

#[derive(Copy, Clone)]
pub enum AssetType {
    Icon,
    Entity,
    Geography,
    Structure,
}

pub struct AssetLoader {
    queue: Option<Vec<(AssetType, DirEntry)>>,
}

impl Default for AssetLoader {
    fn default() -> AssetLoader {
        AssetLoader { queue: None }
    }
}

impl<'a> System<'a> for AssetLoader {
    type SystemData = (specs::Read<'a, GameState>, Write<'a, Assets>);

    fn run(&mut self, (state, mut assets): Self::SystemData) {
        if state.stage != GameStage::LoadingAssets {
            return;
        }
        self.enqueue_assets();
        self.process_queue(&mut assets);
    }
}

impl AssetLoader {
    fn enqueue_assets(&mut self) {
        if self.queue.is_none() {
            let mut queue: Vec<(AssetType, DirEntry)> = Vec::new();
            self.enqueue_directory(&mut queue, AssetType::Entity);
            self.enqueue_directory(&mut queue, AssetType::Icon);
            self.enqueue_directory(&mut queue, AssetType::Geography);
            self.enqueue_directory(&mut queue, AssetType::Structure);
            self.queue = Some(queue);
        }
    }

    fn enqueue_directory(
        &mut self,
        queue: &mut Vec<(AssetType, DirEntry)>,
        template_type: AssetType,
    ) {
        match read_dir(Path::new(&type_dir(template_type))) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(file) => match file.file_type() {
                            Ok(ftype) => {
                                if ftype.is_file() {
                                    queue.push((template_type, file));
                                }
                            }
                            Err(err) => {
                                println!("could not get file type: {}", err);
                            }
                        },
                        Err(err) => {
                            println!("error reading file entry: {}", err);
                        }
                    }
                }
            }
            Err(err) => {
                println!("error reading template directory: {}", err);
            }
        }
    }

    fn process_queue<'a>(&mut self, assets: &mut Write<'a, Assets>) {
        if let Some(queue) = &mut self.queue {
            if let Some((template_type, next)) = queue.pop() {
                println!("reading template from file {:?}", next);
                let path = next.path();
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let file_type = path.extension().unwrap().to_str().unwrap();
                if file_type != "ron" {
                    println!("not a ron file, skipping {:?}", next);
                    return;
                }
                let mut file = File::open(path).expect("error: could not open template file");
                let mut text = String::new();
                file.read_to_string(&mut text)
                    .expect("error: could not read template file");
                match template_type {
                    AssetType::Entity => {
                        let template: EntityTemplate = ron::de::from_str(&text).unwrap();
                        assets.add_entity(name, template);
                    }
                    AssetType::Icon => {
                        let template: Icon = ron::de::from_str(&text).unwrap();
                        assets.add_icon(name, template);
                    }
                    AssetType::Geography => {
                        let template: GeographyTemplate = ron::de::from_str(&text).unwrap();
                        assets.add_geography(name, template);
                    }
                    AssetType::Structure => {
                        let template: StructureTemplate = ron::de::from_str(&text).unwrap();
                        assets.add_structure(name, template);
                    }
                }
            } else {
                assets.ready = true;
                println!("finished loading {} assets: {} entities, {} icons, {} geographies, {} structures.", assets.len(), assets.entity_len(), assets.icon_len(), assets.geography_len(), assets.structure_len());
            }
        }
    }
}
