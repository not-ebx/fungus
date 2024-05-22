use sqlx::{Error, Postgres, Transaction};

pub struct AvatarLookSerializer {
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
}