use fungus_utils::enums::inv_type::InvType;

pub struct ItemInfo {
    pub item_id: i32,
    pub inv_type: InvType,

    pub price: i32,
    pub max_slot: i32,

}
