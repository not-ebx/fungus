use serde::{Deserialize, Serialize};
use sqlx::{Decode, Error, FromRow, Postgres, Row, Transaction, Type};
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::postgres::PgRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
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

impl TryFrom<&PgRow> for AvatarLookSerializer {
    type Error = sqlx::Error;

    fn try_from(row: &PgRow) -> Result<Self, Self::Error> {
        Ok(AvatarLookSerializer {
            id: row.try_get("id")?,
            face: row.try_get("face")?,
            hair: row.try_get("hair")?,
            skin: row.try_get("skin")?,
            job: row.try_get("job")?,
            gender: row.try_get("gender")?,
            weapon_id: row.try_get("weapon_id").unwrap_or(None),
            sub_weapon_id: row.try_get("sub_weapon_id").unwrap_or(None),
            weapon_sticker_id: row.try_get("weapon_sticker_id").unwrap_or(None),
            elf_ear: row.try_get("elf_ear")?,
            ears: row.try_get("ears")?,
            demon_slayer_mark: row.try_get("demon_slayer_mark")?,
        })
    }
}

/*
impl<'r> Decode<'r, sqlx::Postgres> for AvatarLookSerializer {
    fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let row: PgRow = Decode::decode(value)?;
        Ok(AvatarLookSerializer {
            id: row.try_get("id")?,
            face: row.try_get("face")?,
            hair: row.try_get("hair")?,
            skin: row.try_get("skin")?,
            job: row.try_get("job")?,
            gender: row.try_get("gender")?,
            weapon_id: row.try_get("weapon_id").unwrap_or(None),
            sub_weapon_id: row.try_get("sub_weapon_id").unwrap_or(None),
            weapon_sticker_id: row.try_get("weapon_sticker_id").unwrap_or(None),
            elf_ear: row.try_get("elf_ear")?,
            ears: row.try_get("ears")?,
            demon_slayer_mark: row.try_get("demon_slayer_mark")?,
        })
    }
}

 */