use chrono::NaiveDateTime;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::traits::encodable::Encodable;

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

        // TODO
        // Add things like Map ID location, check what is the 'unseen equips' thing too.
        // Ah, and complete this packet lol
    }
}
