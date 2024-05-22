use chrono::NaiveDateTime;

pub struct CharacterSerializer {
    pub id: i32,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub account_id: i32,
    pub avatar_look_id: i32,
    pub character_stats_id: i32,
    pub equipped_inventory: i64,
    pub equip_inventory: i64,
    pub consume_inventory: i64,
    pub install_inventory: i64,
    pub etc_inventory: i64,
    pub cash_inventory: i64,
    pub last_login_id: Option<i32>
}
