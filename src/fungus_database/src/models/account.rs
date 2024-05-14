use chrono::NaiveDateTime;
use sqlx::{Error, FromRow};
use crate::database::get_db;
use crate::models::character::Character;
use crate::models::user::User;
use crate::models::trunk::Trunk;

#[derive(FromRow)]
pub struct Account {
    pub id: i32,                   // auto incr postgresql SERIAL
    pub world_id: i16,             // default is 0
    pub character_slots: i16,      // default is 3
    pub created_at: NaiveDateTime, // default is now()

    // Foreign keys
    pub user_id: i32,              // users table foreign key
    pub trunk_id: i32,                 // trunks table foreign key

}

impl Account {
    pub async fn create_account(user: &User, world_id: i16) -> Result<Account, Error> {
        let pool = &*get_db();
        let trunk = sqlx::query_as!(
            Trunk,
            "INSERT INTO trunks (slots, mesos) VALUES ($1, $2) RETURNING *",
            3,
            0
        ).fetch_one(pool).await?;

        let account = sqlx::query_as!(
            Account,
            "INSERT INTO accounts (world_id, character_slots, user_id, trunk_id) VALUES ($1, $2, $3, $4) RETURNING *",
            world_id, 3, user.id, trunk.id
        ).fetch_one(pool).await?;

        Ok(account)
    }

    pub async fn get_characters(&self) -> Result<&[Character], Error> {
        Ok(&[])
    }
}