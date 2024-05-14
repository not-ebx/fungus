use std::str::FromStr;
use argon2::Config;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use crate::database::get_db;
use crate::models::account::Account;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
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

impl Default for User {
    fn default() -> Self {
        User{
            id: -1,
            username: "".to_string(),
            password: "".to_string(),
            birthday: Default::default(),
            gender: 0,
            nx_cash: 0,
            maple_points: 0,
            vote_points: 0,
            account_type: 0,
            pic: None,
            spw: None,
            ban_expire_date: None,
            ban_reason: None,
            last_login: None,
            created_at: Default::default(),
        }
    }

}

impl User {
    pub fn check_password(&self, pw: String) -> bool {
        argon2::verify_encoded(&self.password.as_str(), pw.as_bytes()).unwrap_or(false)
    }

    // Fetch methods
    pub async fn insert_new_user(name: String, pw: String) ->  Result<User, Error>{
        let pool = &*get_db();
        let crypt_pw = argon2::hash_encoded(
            pw.as_bytes(),
            fungus_utils::constants::server_constants::ARGON_SALT,
            &Config::default(),
        )
        .unwrap_or(pw);

        let date = NaiveDate::from_str("01/01/1990").unwrap_or(NaiveDate::default());
        let new_user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, password, birthday, gender) VALUES ($1, $2, $3, $4) RETURNING *",
            name, crypt_pw, date, 0
        ).fetch_one(pool).await;

        new_user
    }

    pub async fn get_user_by_username(user_username: String) -> Result<User, Error> {
        let pool = &*get_db();
        let user: User = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1",
            user_username
        ).fetch_one(pool).await?;

        Ok(user)
    }

    pub async fn get_login_user(
        user_username: String,
        user_password: String,
    ) -> Result<User, Error> {
        let pool = &*get_db();
        let crypt_pw = argon2::hash_encoded(
            user_password.as_bytes(),
            fungus_utils::constants::server_constants::ARGON_SALT,
            &Config::default(),
        )
        .unwrap_or(user_password);

        let user: User = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1 AND password = $2",
            user_username, crypt_pw
        ).fetch_one(pool).await?;

        Ok(user)
    }

    pub async fn get_user_by_id(id: i32) -> Result<User, Error> {
        let pool = &*get_db();
        let user: User = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        ).fetch_one(pool).await?;

        Ok(user)
    }

    pub async fn get_account(&mut self, world_id: i16) -> Result<Account, Error> {
        let pool = &*get_db();
        let user_id = self.id;
        let acc_res = sqlx::query_as!(
            Account,
            "SELECT * FROM accounts WHERE user_id = $1 AND world_id = $2",
            self.id, world_id
        ).fetch_one(pool).await;

        match acc_res {
            Ok(acc) => Ok(acc),
            Err(_) => {
                Account::create_account(&self, world_id).await
            }
        }
    }
}
