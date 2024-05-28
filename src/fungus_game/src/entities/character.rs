use chrono::{NaiveDateTime};
use fungus_database::serializers::character_select_serializer::CharacterSelectSerializer;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::traits::encodable::Encodable;
use fungus_utils::enums::inv_type::InvType;
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

    equipped_inventory: Inventory<Equipment>,
    equip_inventory: Inventory<Equipment>,
    consume_inventory: Inventory<Item>,
    etc_inventory: Inventory<Item>,
    install_inventory: Inventory<Item>,
    cash_inventory: Inventory<Item>,
}

impl Encodable for Character {
    fn encode(&self, out_packet: &mut OutPacket) {
        out_packet.write_int(self.id);
        out_packet.write_exact_string(self.character_stats.name.clone(), 13);
        out_packet.write_byte(self.avatar_look.gender as u8);
        out_packet.write_byte(self.avatar_look.skin as u8);
        out_packet.write_int(self.avatar_look.face);
        out_packet.write_int(self.avatar_look.hair);
        // Pets
        for _ in 0..3 {
            out_packet.write_long(0);
        }
        out_packet.write(&self.character_stats);
        out_packet.write(&self.avatar_look);
    }
}

impl From<CharacterSelectSerializer> for Character {
    fn from(value: CharacterSelectSerializer) -> Self {
        Character {
            id: value.character.id,
            character_stats: CharacterStats::from(value.character_stats),
            avatar_look: AvatarLook::from(value.avatar_look),
            deleted_at: value.character.deleted_at,
            created_at: value.character.created_at,
            equipped_inventory: Inventory::new(value.character.equipped_inventory, 0, InvType::Equipped),
            equip_inventory: Inventory::new(value.character.equip_inventory, 0, InvType::Equip),
            consume_inventory: Inventory::new(value.character.consume_inventory, 0, InvType::Consume),
            etc_inventory: Inventory::new(value.character.etc_inventory, 0, InvType::Etc),
            install_inventory: Inventory::new(value.character.install_inventory, 0, InvType::Install),
            cash_inventory: Inventory::new(value.character.cash_inventory, 0, InvType::Cash),
        }
    }
}

impl Character {
    pub fn new() {
    }
}



