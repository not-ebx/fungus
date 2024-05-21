use chrono::NaiveDateTime;
use fungus_utils::enums::inv_type::InvType;
use fungus_utils::enums::item_type::ItemType;
use crate::serializers::item_serializer::ItemSerializer;

#[derive(Clone)]
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

    pub is_trade_blocked: bool,
    // times
    pub expires_at: Option<NaiveDateTime>
}

impl Into<ItemSerializer> for Item {
    fn into(self) -> ItemSerializer {
        ItemSerializer {
            id: self.id,
            bag_index: self.bag_index,
            cash_serial_number: self.cash_serial_number,
            inv_type: self.inv_type,
            item_type: self.item_type,
            is_cash: self.is_cash,
            item_id: self.item_id,
            owner: self.owner.clone(),
            quantity: self.quantity,
            inventory_id: None, // TODO: Get the values from db when serializing..?
            trunk_id: None,
            expires_at: self.expires_at,
        }
    }
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
            is_trade_blocked: false,
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