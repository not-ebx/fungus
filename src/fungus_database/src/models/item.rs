use chrono::NaiveDateTime;
use crate::enums::inv_type::InvType;
use crate::enums::item_type::ItemType;

pub struct Item {
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

impl Default for Item {
    fn default() -> Self {
        Item {
            id: 0,
            bag_index: 0,
            cash_serial_number: 0,
            inv_type: InvType::None,
            item_type: ItemType::Item,
            is_cash: false,
            item_id: 0,
            owner: "".to_string(),
            quantity: 0,
            inventory_id: None,
            trunk_id: None,
            expires_at: None,
        }
    }
}

impl Item {
    pub fn new_default(item_id: i32, item_type: ItemType) -> Self {
        let mut item = Self::default();
        item.item_id = item_id;
        item.item_type = item_type;

        item
    }
}