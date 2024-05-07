use argon2::Config;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::insert_into;
use diesel::prelude::*;
use crate::database::get_db;
use crate::schema::users::{password, username};
use crate::schema::users::dsl::users;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
struct AutoregisterUser {
    pub username: String,
    pub password: String,
    pub birthday: NaiveDate,
    pub gender: i16,
}

impl User {
    pub fn check_password(&self, pw: String) -> bool {
        argon2::verify_encoded(
            &self.password.as_str(),
            pw.as_bytes()
        ).unwrap_or(false)
    }


    // Fetch methods
    pub fn insert_new_user(name: String, pw: String) -> QueryResult<Self> {
        let mut conn = get_db();

        let crypt_pw = argon2::hash_encoded(
            pw.as_bytes(),
            fungus_utils::constants::server_constants::ARGON_SALT,
            &Config::default()
        ).unwrap_or(pw);

        let new_user = AutoregisterUser{
            username: name,
            password: crypt_pw,
            birthday: Default::default(),
            gender: 0,
        };

        insert_into(users)
            .values(&new_user)
            .get_result(&mut conn)
    }

    pub fn get_user_by_username(user_username: String) -> Result<User, diesel::result::Error> {
        let mut conn = get_db();
        users
            .filter(username.eq(user_username))
            .first(&mut conn)
    }

    pub fn get_login_user(user_username: String, user_password: String) -> Result<User, diesel::result::Error> {
        let mut conn = get_db();

        let crypt_pw = argon2::hash_encoded(
            user_password.as_bytes(),
            fungus_utils::constants::server_constants::ARGON_SALT,
            &Config::default()
        ).unwrap_or(user_password);

        users
            .filter(username.eq(user_username))
            .filter(password.eq(crypt_pw))
            .first(&mut conn)
    }

    pub fn get_user_by_id(id: i32) -> Result<User, diesel::result::Error> {
        let mut conn = get_db();
        users
            .find(id)
            .first(&mut conn)
    }
}

