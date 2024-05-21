use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserSerializer {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub birthday: NaiveDate,
    pub gender: i16,
    pub nx_cash: i32,
    pub maple_points: i32,
    pub vote_points: i32,
    pub account_type: i16, // Byte Flags, behave as permission
    pub pic: Option<i16>,
    pub spw: Option<String>,
    pub ban_expire_date: Option<NaiveDateTime>,
    pub ban_reason: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}