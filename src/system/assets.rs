use specs::{System, Write};
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use std::path::Path;

use crate::constants::ENTITY_TEMPLATE_DIR;
use crate::resource::{
    EntityTemplate, GameStage, GameState, GeographyTemplate, IconTemplate, StructureTemplate,
    TemplateType, Templates,
};

fn type_dir<'a>(template_type: TemplateType) -> &'a str {
    match template_type {
        TemplateType::Icon => "icons",
        TemplateType::Entity => "entities",
        TemplateType::Geography => "geographies",
        TemplateType::Structure => "structures",
    }
}

pub struct AssetLoader {
    queue: Option<Vec<(TemplateType, DirEntry)>>,
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
        self.enqueue_assets();
        self.process_queue(&mut templates);
    }
}

impl AssetLoader {
    fn enqueue_assets(&mut self) {
        if self.queue.is_none() {
            let mut queue: Vec<(TemplateType, DirEntry)> = Vec::new();
            self.enqueue_directory(&mut queue, TemplateType::Entity);
            self.enqueue_directory(&mut queue, TemplateType::Icon);
            self.enqueue_directory(&mut queue, TemplateType::Geography);
            self.enqueue_directory(&mut queue, TemplateType::Structure);
            self.queue = Some(queue);
        }
    }

    fn enqueue_directory(
        &mut self,
        queue: &mut Vec<(TemplateType, DirEntry)>,
        template_type: TemplateType,
    ) {
        match read_dir(Path::new(ENTITY_TEMPLATE_DIR).join(type_dir(template_type))) {
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

    fn process_queue<'a>(&mut self, templates: &mut Write<'a, Templates>) {
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
                    TemplateType::Entity => {
                        let template: EntityTemplate = ron::de::from_str(&text).unwrap();
                        templates.add_entity(name, template);
                    }
                    TemplateType::Icon => {
                        let template: IconTemplate = ron::de::from_str(&text).unwrap();
                        templates.add_icon(name, template);
                    }
                    TemplateType::Geography => {
                        let template: GeographyTemplate = ron::de::from_str(&text).unwrap();
                        templates.add_geography(name, template);
                    }
                    TemplateType::Structure => {
                        let template: StructureTemplate = ron::de::from_str(&text).unwrap();
                        templates.add_structure(name, template);
                    }
                }
            } else {
                templates.ready = true;
                println!("finished loading {} templates: {} entities, {} icons, {} geographies, {} structures.", templates.len(), templates.entity_len(), templates.icon_len(), templates.geography_len(), templates.structure_len());
            }
        }
    }
}
