use std::error::Error;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, Postgres, Row, Type};
use sqlx::postgres::{PgRow, PgValueRef};

#[derive(Debug, FromRow, Serialize, Deserialize)]
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

impl TryFrom<&PgRow> for CharacterSerializer {
    type Error = sqlx::Error;

    fn try_from(row: &PgRow) -> Result<Self, Self::Error> {
        Ok(CharacterSerializer {
            id: row.try_get("id")?,
            deleted_at: row.try_get("deleted_at")?,
            created_at: row.try_get("created_at")?,
            account_id: row.try_get("account_id")?,
            avatar_look_id: row.try_get("avatar_look_id")?,
            character_stats_id: row.try_get("character_stats_id")?,
            equipped_inventory: row.try_get("equipped_inventory")?,
            equip_inventory: row.try_get("equip_inventory")?,
            consume_inventory: row.try_get("consume_inventory")?,
            install_inventory: row.try_get("install_inventory")?,
            etc_inventory: row.try_get("etc_inventory")?,
            cash_inventory: row.try_get("cash_inventory")?,
            last_login_id: row.try_get("last_login_id").unwrap_or(None),
        })
    }
}

/*
impl<'r> Decode<'r, Postgres> for CharacterSerializer {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let row: PgRow = sqlx::decode::Decode::decode(value)?;
        Ok(CharacterSerializer {
            id: row.try_get("id")?,
            deleted_at: row.try_get("deleted_at")?,
            created_at: row.try_get("created_at")?,
            account_id: row.try_get("account_id")?,
            avatar_look_id: row.try_get("avatar_look_id")?,
            character_stats_id: row.try_get("character_stats_id")?,
            equipped_inventory: row.try_get("equipped_inventory")?,
            equip_inventory: row.try_get("equip_inventory")?,
            consume_inventory: row.try_get("consume_inventory")?,
            install_inventory: row.try_get("install_inventory")?,
            etc_inventory: row.try_get("etc_inventory")?,
            cash_inventory: row.try_get("cash_inventory")?,
            last_login_id: row.try_get("last_login_id").unwrap_or(None),
        })
    }
}

 */
