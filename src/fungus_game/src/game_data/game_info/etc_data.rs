use std::io::Error;
use std::collections::HashSet;
use log::info;
use rust_nx::nx_file::NXFile;
use rust_nx::nx_node::{NXNode, NXNodeData, NXNodeType};
use fungus_utils::constants::game_constants::NX_FILES_DIR;

pub struct EtcData {
    pub starting_items: HashSet<i32>,
}

impl EtcData {

    pub fn new() -> Self {
        EtcData {
            starting_items: Default::default()
        }
    }

    pub fn load_all(&mut self) -> Result<(), Error> {
        info!("Loading Starting items");
        self.load_starting_items()?;
        info!("Loaded {} items", self.starting_items.len());

        Ok(())
    }

    fn load_starting_items(&mut self) -> Result<(), Error> {
        let files_loc = NX_FILES_DIR.to_string() + "/Etc.nx";
        let etc_nx = NXFile::new(files_loc.as_str()).unwrap();

        let make_char_info = etc_nx.resolve("MakeCharInfo.img").expect("FAILED");

        self.search_starting_item(&etc_nx, make_char_info);

        Ok(())
    }

    fn search_starting_item(&mut self, etc_nx: &NXFile, node: &NXNode) {
        let children = etc_nx.get_node_children(node);
        for child in children.iter() {
            let node_name = child.name.clone().parse::<i32>();
            match node_name {
                Ok(name) => {
                    if child.ntype.eq(&NXNodeType::Long) {
                        let item_id: i32 = child.data.clone().into();
                        self.starting_items.insert(item_id);
                    } else {
                        self.search_starting_item(etc_nx, child);
                    }
                }
                Err(_) => {
                    self.search_starting_item(etc_nx, child);
                }
            }

        }
    }
}