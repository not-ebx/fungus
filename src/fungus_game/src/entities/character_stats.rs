use chrono::NaiveDateTime;
use log::warn;
use fungus_database::serializers::character_stats_serializer::CharacterStatsSerializer;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::traits::encodable::Encodable;
use crate::game_data::game_info::job_utilities::is_extend_sp_job;

pub struct CharacterStats {
    // Character Details
    id: i32,
    pub name: String,
    gender: i16,
    job: i32,
    sub_job: i32,
    level: i32, // Defaults at 1
    exp: i64,
    mesos: i64,
    ap: i32,
    sp: i32,
    //extendSP ==> Need a table for this :)
    pub map_id: i64,
    pub portal: i32,

    // Character Stats
    str: i32,
    dex: i32,
    int: i32,
    luk: i32,
    hp: i32,
    max_hp: i32,
    mp: i32,
    max_mp: i32,
    pop: i32,

    // Profession & traits stats
    willpower_level: i32,
    willpower_exp: i32,
    charm_level: i32,
    charm_exp: i32,
    insight_level: i32,
    insight_exp: i32,
    ambition_level: i32,
    ambition_exp: i32,
    empathy_level: i32,
    empathy_exp: i32,
    diligence_level: i32,
    diligence_exp: i32,

    craft_level: i32,
    craft_exp: i32,
    fatigue: i32,
    honor_exp: i32,

    // Pvp stats
    pvp_exp: i32,
    pvp_grade: i32,
    pvp_mode_level: i32,
    pvp_mode_type: i32,
    pvp_point: i32,

    // Times
    pop_available_at: NaiveDateTime, //Defaults as current time
    fatigue_updated_at: NaiveDateTime, //Defaults as current time

}

impl Encodable for CharacterStats {
    fn encode(&self, out_packet: &mut OutPacket) {
        out_packet.write_byte(self.level as u8);
        out_packet.write_short(self.job as i16);
        out_packet.write_short(self.str as i16);
        out_packet.write_short(self.dex as i16);
        out_packet.write_short(self.int as i16);
        out_packet.write_short(self.luk as i16);
        out_packet.write_int(self.hp);
        out_packet.write_int(self.max_hp);
        out_packet.write_int(self.mp);
        out_packet.write_int(self.max_mp);
        out_packet.write_short(self.ap as i16);

        if is_extend_sp_job(self.job as i16) {
            // TODO Encode Extended SP
            warn!("Tried to encode a extended_sp job. Client will crash.");
        } else {
            out_packet.write_short(self.sp as i16);
        }

        // TODO
        // Add things like Map ID location, check what is the 'unseen equips' thing too.
        // Ah, and complete this packet lol
        out_packet.write_int(self.exp as i32);
        out_packet.write_int(self.pop);
        out_packet.write_int(0); // Gach exp.
        out_packet.write_int(self.map_id as i32);
        out_packet.write_byte(self.portal as u8);
        out_packet.write_int(0); // Online time in seconds (?)
        out_packet.write_short(self.sub_job as i16);

        // Here we check if it's demon, but we won't do that now.
        // if demon
        //encode int (demon face acc id)

        out_packet.write_byte(self.fatigue as u8);
        out_packet.write_int(0); // TODO Fatigue update; should be int

        out_packet.write_int(self.charm_exp);
        out_packet.write_int(self.insight_exp);
        out_packet.write_int(self.willpower_exp);
        out_packet.write_int(self.craft_exp);
        out_packet.write_int(self.diligence_exp); // Unsure
        out_packet.write_int(self.empathy_exp); // Unsure
        //out_packet.write_int(self.ambition_exp); // Unsure

        // Traits daily limits. TODO
        out_packet.write_short(0);
        out_packet.write_short(0);
        out_packet.write_short(0);
        out_packet.write_short(0);
        out_packet.write_short(0);
        out_packet.write_short(0);

        // PVP Stuff.
        out_packet.write_int(self.pvp_exp);
        out_packet.write_byte(self.pvp_grade as u8);
        out_packet.write_int(self.pvp_point);
        out_packet.write_byte(5); // ?
        out_packet.write_int(0);

        // Last logout. TODO
        out_packet.write_long(0);

    }
}

impl From<CharacterStatsSerializer> for CharacterStats {
    fn from(value: CharacterStatsSerializer) -> Self {
        CharacterStats {
            id: value.id,
            name: value.name,
            gender: value.gender,
            job: value.job,
            sub_job: value.sub_job,
            level: value.level,
            exp: value.exp,
            mesos: value.mesos,
            ap: value.ap,
            sp: value.sp,
            map_id: value.map_id,
            portal: value.portal,
            str: value.str,
            dex: value.dex,
            int: value.int,
            luk: value.luk,
            hp: value.hp,
            max_hp: value.max_hp,
            mp: value.mp,
            max_mp: value.max_mp,
            pop: value.pop,
            willpower_level: value.willpower_level,
            willpower_exp: value.willpower_exp,
            charm_level: value.charm_level,
            charm_exp: value.charm_exp,
            insight_level: value.insight_level,
            insight_exp: value.insight_exp,
            ambition_level: value.ambition_level,
            ambition_exp: value.ambition_exp,
            empathy_level: value.empathy_level,
            empathy_exp: value.empathy_exp,
            diligence_level: value.diligence_level,
            diligence_exp: value.diligence_exp,
            craft_level: value.craft_level,
            craft_exp: value.craft_exp,
            fatigue: value.fatigue,
            honor_exp: value.honor_exp,
            pvp_exp: value.pvp_exp,
            pvp_grade: value.pvp_grade,
            pvp_mode_level: value.pvp_mode_level,
            pvp_mode_type: value.pvp_mode_type,
            pvp_point: value.pvp_point,
            pop_available_at: value.pop_available_at,
            fatigue_updated_at: value.fatigue_updated_at
        }
    }
}
