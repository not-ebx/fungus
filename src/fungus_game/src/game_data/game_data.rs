use std::sync::Arc;
use once_cell::sync::Lazy;
use fungus_utils::enums::inv_type::InvType;
use fungus_utils::utility::item_utility::is_equipment;
use fungus_game::::game_info::etc_data::EtcData;
use fungus_game::::item_data::ItemData;
use crate::entities::equipment::Equipment;
use crate::entities::item::Item;

pub struct GameData {
    // Stuff
    pub item_data: ItemData,
    pub etc_data: EtcData,
}

impl GameData {
    pub fn new() -> Self {
        let mut item_data= ItemData::new();
        item_data.load_all().expect("Dead.");

        let mut etc_data = EtcData::new();
        etc_data.load_all().expect("Failed to load etc nx");

        GameData {
            item_data,
            etc_data,
        }
    }

    pub fn fetch_equipment(&self, id: i32) -> Option<Equipment>{
        match self.item_data.equipments.get(&id) {
            Some(&eqp) => Some(eqp.clone()),
            None => None
        }
    }

    pub fn fetch_item(&self, id: i32) -> Option<Item>{
        match self.item_data.item_info.get(&id) {
            Some(&eqp) => Some(eqp.clone()),
            None => None
        }
    }

    pub fn get_inv_type(&self, id: i32) -> InvType {
        if(is_equipment(id)) {
            InvType::Equip
        } else {
            self.item_data.get_inv_type(id)
        }
    }
}


pub static GAME_DATA: Lazy<Arc<GameData>> = Lazy::new(|| {
    Arc::new(GameData::new())
});