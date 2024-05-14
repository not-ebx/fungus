use crate::enums::inv_type::InvType;

pub struct Inventory {
    id: i64,
    slots: i16,
    inv_type: InvType
}