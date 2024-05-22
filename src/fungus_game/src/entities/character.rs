use chrono::{NaiveDateTime};
use sqlx::{Error, Postgres};
use fungus_utils::constants::game_constants::{DEFAULT_INVENTORY_SIZE, MAX_INVENTORY_SIZE};
use fungus_utils::enums::inv_type::InvType;
use crate::entities::avatar_look::AvatarLook;
use crate::entities::character_stats::CharacterStats;
use crate::entities::equipment::Equipment;
use crate::entities::inventory::Inventory;
use crate::entities::item::Item;

trait AddItemToInventory<T> {
    async fn add_item_to_inventory(&mut self, item: T) -> Result<(), sqlx::Error>;
}

pub struct Character {
    pub id: i32,
    pub stats: CharacterStats,
    pub look: AvatarLook,
    // Times
    deleted_at: Option<NaiveDateTime>, // Soft delete!
    created_at: NaiveDateTime, // Defaults at now()


    equip_inventory: Inventory<Equipment>,
    use_inventory: Inventory<Item>,
    etc_inventory: Inventory<Item>,
    install_inventory: Inventory<Item>,
    cash_inventory: Inventory<Item>,
}

impl Character {
    pub fn new() {
    }


}



