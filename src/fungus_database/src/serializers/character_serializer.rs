use chrono::NaiveDateTime;

pub struct CharacterSerializer {
    pub id: i32,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub account_id: i32,
    pub avatar_look_id: i32,
    pub character_stats_id: i32,
    pub equipped_inventory: i32,
    pub equip_inventory: i32,
    pub consume_inventory: i32,
    pub install_inventory: i32,
    pub etc_inventory: i32,
    pub cash_inventory: i32,
    pub last_login_id: i32
}
