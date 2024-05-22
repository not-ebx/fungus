use fungus_utils::enums::inv_type::InvType;

pub struct InventorySerializer {
    pub id: i64,
    pub slots: i16,
    pub inv_type: InvType
}