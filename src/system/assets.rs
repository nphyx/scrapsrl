use specs::{System, Write};
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use std::path::Path;

use crate::constants::ENTITY_TEMPLATE_DIR;
use crate::resource::{EntityTemplate, GameStage, GameState, Templates};

pub struct AssetLoader {
    queue: Option<Vec<DirEntry>>,
}

impl Default for AssetLoader {
    fn default() -> AssetLoader {
        AssetLoader { queue: None }
    }
}

impl<'a> System<'a> for AssetLoader {
    type SystemData = (specs::Read<'a, GameState>, Write<'a, Templates>);

    fn run(&mut self, (state, mut templates): Self::SystemData) {
        if state.stage != GameStage::LoadingAssets {
            return;
        }
        if self.queue.is_none() {
            let mut queue = Vec::new();
            match read_dir(Path::new(ENTITY_TEMPLATE_DIR)) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(file) => match file.file_type() {
                                Ok(ftype) => {
                                    println!("enqueueing {:?}", file);
                                    if ftype.is_file() {
                                        queue.push(file);
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
            self.queue = Some(queue);
        }
        if let Some(queue) = &mut self.queue {
            if let Some(next) = queue.pop() {
                println!("reading template at {:?}", next);
                let path = next.path();
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let mut file = File::open(path).expect("error: could not open template file");
                let mut text = String::new();
                file.read_to_string(&mut text)
                    .expect("error: could not read template file");
                let template: EntityTemplate = ron::de::from_str(&text).unwrap();
                templates.add(name, template);
            } else {
                templates.ready = true;
                println!("finished loading templates.");
            }
        }
    }
}
