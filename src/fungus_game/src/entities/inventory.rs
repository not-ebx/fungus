use fungus_utils::enums::inv_type::InvType;

pub struct Inventory<T> {
    id: i64,
    slots: i16,
    inv_type: InvType,
    items: Vec<T>
}

impl<T> Inventory<T> {
    pub fn new(id: i64, slots: i16, inv_type: InvType) -> Self {
        Inventory {
            id,
            slots,
            inv_type,
            items: vec![],
        }
    }
}