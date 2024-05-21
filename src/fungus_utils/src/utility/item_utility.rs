use crate::enums::inv_type::InvType;

fn get_item_prefix(id: i32) -> i32 {
    return id / 10_000;
}

pub fn is_equipment(id: i32) -> bool {
    return id / 1_000_000 == 1;
}

