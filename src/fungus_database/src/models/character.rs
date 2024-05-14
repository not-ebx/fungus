use chrono::{NaiveDateTime};

pub struct Character {
    pub id: i32,

    // Times
    deleted_at: Option<NaiveDateTime>, // Soft delete!
    created_at: NaiveDateTime, // Defaults at now()

    // Foreign Keys
    account_id: i32, // foreign key to the accounts table.
    avatar_look_id: i32, // AvatarLook table reference
    character_stats_id: i32, // CharacterStats table reference
}

impl Character {
    pub fn new() {

    }
}



