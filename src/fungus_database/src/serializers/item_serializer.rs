use std::future::IntoFuture;
use chrono::NaiveDateTime;
use fungus_utils::enums::inv_type::InvType;
use fungus_utils::enums::item_type::ItemType;
use crate::models::equipment::Equipment;
use crate::models::item::Item;

pub struct ItemSerializer {
    pub id: i64, //big int postgres
    pub bag_index: i32,
    pub cash_serial_number: i64,
    pub inv_type: InvType, // It's a SmallInt internally
    pub item_type: ItemType,
    pub is_cash: bool,
    pub item_id: i32, //game item id, related to game files not database. It's just a number, NOT A FOREIGN KEY@
    pub owner: String, //A string with a name
    pub quantity: i32, // by default 1

    // Inventory details. Could have inventory_id OR trunk_id, but never both.
    pub inventory_id: Option<i64>, // Foreign key to inventories table
    pub trunk_id: Option<i32>, // Foreign key to trunks table

    // times
    pub expires_at: Option<NaiveDateTime>
}
