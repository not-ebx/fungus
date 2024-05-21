pub struct EquipmentSerializer {
    pub item_id: i64, // items table foreign key and key for this table too, since there's no duplicates.

    pub attribute: i16,
    pub attack_speed: i32,

    // Stat Requirement
    pub req_str: i32,
    pub req_dex: i32,
    pub req_int: i32,
    pub req_luk: i32,
    pub req_level: i32,
    pub req_pop: i32,
    pub req_job: i32,

    // Stat increase
    pub inc_hp: i16,
    pub inc_mp: i16,
    pub inc_str: i32,
    pub inc_dex: i32,
    pub inc_int: i32,
    pub inc_luk: i32,
    pub inc_accuracy: i16,
    pub inc_craft: i16, // AKA hands
    pub inc_evasion: i16, // AKA avoid
    pub inc_jump: i16,
    pub inc_speed: i16,
    pub inc_mad: i16, // AD is Attack
    pub inc_mdd: i16, // DD is defense
    pub inc_pad: i16,
    pub inc_pdd: i16,
    pub inc_ied: i16, //aka MDR
    pub inc_total_damage: i16, // AKA DAMr
    pub inc_pvp_damage: i16,
    pub inc_reduce_req: i16, // Reduce level req
    pub inc_boss_damage_range: i16, // known as bdR in wz

    // Item upgrade details
    pub total_upgrade_count: i16,
    pub current_upgrade_count: i16, // cuc
    pub enchant_count: i16, //chuc
    pub inc_upgrade_count: i16, // Hammers used, aka iuc

    // Misc
    pub charm_exp: i32,
    pub exp: i16,
    pub item_level: i16,
    pub durability: i16,
    pub durability_max: i16,
    pub price: i32,
    pub serial_number: i64,
    pub i_slot: String, // Varchar, emtpy as default
    pub v_slot: String, // Varchar, empty ad default
    pub ps_enchant: i16, // Final Strike
    pub set_id: i32, // Equipment set id. In-game data, too.
    pub android: i32,
    pub android_grade: i32,

    // Details. Bools are false by default, unless stated.
    pub is_trade_blocked: bool,
    pub is_unique: bool, // In-game uniqueness value, not related to database
    pub is_potable: bool, // is_potable is TRUE by default
    pub is_expired_on_logout: bool,
    pub is_boss_reward: bool,
    pub has_fixed_potential: bool,
    pub is_sellable: bool, // is_sellable is TRUE by default
    pub is_sokable: bool, // Can use Siccors of Karma (sok)
    pub is_superior_equip: bool,
}