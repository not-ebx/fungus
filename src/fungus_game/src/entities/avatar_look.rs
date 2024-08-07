use std::ptr::eq;
use fungus_database::serializers::avatar_look_serializer::AvatarLookSerializer;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::traits::encodable::Encodable;
use crate::entities::equipment::Equipment;
use crate::game_data::game_info::item_utilities::{BodyPart, get_body_part_from_item};

pub struct AvatarLook {
    pub id: i32,
    pub face: i32,
    pub hair: i32,
    pub skin: i32,
    pub job: i32,
    pub gender: i16,
    pub weapon_id: Option<i32>,
    pub sub_weapon_id: Option<i32>,
    pub weapon_sticker_id: Option<i32>,
    pub elf_ear: bool,
    pub ears: i32,

    // Face Accessories
    pub demon_slayer_mark: i32,

    // Non-sql related stuff
    pub visible_equipment: Vec<i32>,
    pub invisible_equipment: Vec<i32>

}

impl From<AvatarLookSerializer> for AvatarLook {
    fn from(value: AvatarLookSerializer) -> Self {
        let visible_equipment = {
            let mut eqp_vec = vec![];
            if let Some(weapon_id) = value.weapon_id {
                eqp_vec.push(weapon_id);
            }

            if let Some(sub_weapon_id) = value.sub_weapon_id {
                eqp_vec.push(sub_weapon_id);
            }

            eqp_vec
        };

        AvatarLook {
            id: value.id,
            face: value.face,
            hair: value.hair,
            skin: value.skin,
            job: value.job,
            gender: value.gender,
            weapon_id: value.weapon_id,
            sub_weapon_id: value.sub_weapon_id,
            weapon_sticker_id: value.weapon_sticker_id,
            elf_ear: value.elf_ear,
            ears: value.ears,
            demon_slayer_mark: value.demon_slayer_mark,
            visible_equipment,
            invisible_equipment: vec![]
        }
    }
}

impl Encodable for AvatarLook {
    fn encode(&self, out_packet: &mut OutPacket) {
        out_packet.write_byte(self.gender as u8);
        out_packet.write_byte(self.skin as u8);
        out_packet.write_int(self.face);
        //out_packet.write_int(self.job);
        out_packet.write_byte(1); // Mega? Idk what this is
        out_packet.write_int(self.hair);

        for eqp_id in &self.visible_equipment {
            if let Some(body_part) = get_body_part_from_item(eqp_id.clone()) {
                out_packet.write_byte(body_part as u8);
                out_packet.write_int(eqp_id.clone());
            }
        }
        out_packet.write_byte(0xFF);

        // Invisible Equipment: Things like monsterbook, pendants, etc.
        for eqp_id in &self.invisible_equipment {
            // hair is the default
            let body_part = get_body_part_from_item(eqp_id.clone()).unwrap_or(BodyPart::Hair);
            out_packet.write_byte(body_part as u8);
            out_packet.write_int(eqp_id.clone());
        }
        out_packet.write_byte(0xFF);

        out_packet.write_int(self.weapon_sticker_id.unwrap_or(0));
        // Pets
        for _ in 0..3 {
            out_packet.write_int(0);
        }
    }
}