use crate::enums::inv_type::InvType;

pub struct ItemInfo {
    pub item_id: i32,
    pub inv_type: InvType,
    pub price: i32,
    pub max_slot: i32,

    pub is_trade_blocked: bool,
}

impl ItemInfo {
    pub fn new(item_id: i32) -> Self {
        ItemInfo{
            item_id,
            inv_type: InvType::None,
            price: 0,
            max_slot: 0,
            is_trade_blocked: false,
        }
    }
}