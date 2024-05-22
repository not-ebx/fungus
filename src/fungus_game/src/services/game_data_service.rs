use tokio::time::Instant;
use fungus_utils::enums::inv_type::InvType;
use fungus_utils::{fg_printc_error, fg_printc_info};
use fungus_utils::utility::item_utility::is_equipment;
use crate::entities::equipment::Equipment;
use crate::entities::item::Item;
use crate::game_data::etc_data::EtcData;
use crate::game_data::item_data::ItemData;

pub struct GameDataService {
    pub item_data: ItemData,
    pub etc_data: EtcData,
}

impl GameDataService {
    pub fn new() -> Self {
        let mut item_data= ItemData::new();
        let mut timer = Instant::now();
        fg_printc_info!("Loading Items");
        if let Err(_) = item_data.load_all() {
            fg_printc_error!("Failed to load item data");
            panic!();
        }
        let mut curr_time = Instant::now();
        let mut duration = curr_time - timer;
        fg_printc_info!("Loaded all items in {}ms", duration.as_millis());

        timer = Instant::now();
        fg_printc_info!("Loading Etc");
        let mut etc_data = EtcData::new();
        if let Err(_) = etc_data.load_all() {
            fg_printc_error!("Failed to load etc nx data");
            panic!();
        }
        curr_time = Instant::now();
        duration = curr_time - timer;
        fg_printc_info!("Loaded etc data in {}ms", duration.as_millis());

        GameDataService {
            item_data,
            etc_data,
        }
    }


    pub fn fetch_item(&self, id: i32) -> Option<Item> {
        self.item_data.fetch_item(id)
    }

    pub fn fetch_equipment(&self, id: i32) -> Option<Equipment> {
        self.item_data.fetch_equipment(id)
    }

    pub fn get_inv_type(&self, id: i32) -> InvType {
        if(is_equipment(id)) {
            InvType::Equip
        } else {
            self.item_data.get_inv_type(id)
        }
    }
}