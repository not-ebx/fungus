use std::str::FromStr;
use argon2::Config;
use chrono::NaiveDate;
use sqlx::{Error, PgPool};
use crate::serializers::user_serializer::UserSerializer;

pub struct UserDAO;

impl UserDAO {
    pub fn new() -> Self {
        UserDAO{}
    }
    pub async fn insert_new_user(&self, pool: &PgPool, name: String, pw: String) -> Result<UserSerializer, sqlx::Error> {
        let crypt_pw = argon2::hash_encoded(
            pw.as_bytes(),
            fungus_utils::constants::server_constants::ARGON_SALT,
            &Config::default(),
        )
            .unwrap_or(pw);

        let date = NaiveDate::from_str("01/01/1990").unwrap_or(NaiveDate::default());
        sqlx::query_as!(
            UserSerializer,
            "INSERT INTO users (username, password, birthday, gender) VALUES ($1, $2, $3, $4) RETURNING *",
            name, crypt_pw, date, 0
        ).fetch_one(pool).await
    }

    pub async fn get_user_by_username(&self, pool: &PgPool, user_username: String) -> Result<UserSerializer, Error> {
        sqlx::query_as!(
            UserSerializer,
            "SELECT * FROM users WHERE username = $1",
            user_username
        ).fetch_one(pool).await
    }

    pub async fn get_login_user(
        &self,
        pool: &PgPool,
        user_username: String,
        user_password: String,
    ) -> Result<UserSerializer, Error> {
        let crypt_pw = argon2::hash_encoded(
            user_password.as_bytes(),
            fungus_utils::constants::server_constants::ARGON_SALT,
            &Config::default(),
        )
            .unwrap_or(user_password);

        sqlx::query_as!(
            UserSerializer,
            "SELECT * FROM users WHERE username = $1 AND password = $2",
            user_username, crypt_pw
        ).fetch_one(pool).await
    }

    pub async fn get_user_by_id(&self, pool: &PgPool,id: i32) -> Result<UserSerializer, Error> {
        sqlx::query_as!(
            UserSerializer,
            "SELECT * FROM users WHERE id = $1",
            id
        ).fetch_one(pool).await
    }

}