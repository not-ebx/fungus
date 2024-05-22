use chrono::{NaiveDateTime};
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::traits::encodable::Encodable;
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
    pub character_stats: CharacterStats,
    pub avatar_look: AvatarLook,
    // Times
    deleted_at: Option<NaiveDateTime>, // Soft delete!
    created_at: NaiveDateTime, // Defaults at now()


    equip_inventory: Inventory<Equipment>,
    use_inventory: Inventory<Item>,
    etc_inventory: Inventory<Item>,
    install_inventory: Inventory<Item>,
    cash_inventory: Inventory<Item>,
}

impl Encodable for Character {
    fn encode(&self, out_packet: &mut OutPacket) {
        out_packet.write(&self.avatar_look);
        out_packet.write_int(self.id);
        out_packet.write_string(self.character_stats.name.clone());
        out_packet.write_byte(self.avatar_look.gender as u8);
        out_packet.write_byte(self.avatar_look.skin as u8);
        out_packet.write_int(self.avatar_look.face);
        out_packet.write_int(self.avatar_look.hair);

        // Pets
        for _ in 0..3 {
            out_packet.write_long(0);
        }

        out_packet.write(&self.character_stats);
    }
}

impl Character {
    pub fn new() {
    }


}



