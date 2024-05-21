use chrono::{NaiveDate, NaiveDateTime};
use fungus_database::serializers::user_serializer::UserSerializer;

pub struct User {
    pub id: i32,
    pub username: String,
    pub birthday: NaiveDate,
    pub gender: i16,
    pub nx_cash: i32,
    pub maple_points: i32,
    pub vote_points: i32,
    pub account_type: i16, // Byte Flags, behave as permission
    pub created_at: NaiveDateTime,
}

impl Default for User {
    fn default() -> Self {
        User{
            id: -1,
            username: "".to_string(),
            birthday: Default::default(),
            gender: 0,
            nx_cash: 0,
            maple_points: 0,
            vote_points: 0,
            account_type: 0,
            created_at: Default::default(),
        }
    }

}

impl From<UserSerializer> for User {
    fn from(value: UserSerializer) -> Self {
        User {
            id: value.id,
            username: value.username,
            birthday: value.birthday,
            gender: value.gender,
            nx_cash: value.nx_cash,
            maple_points: value.maple_points,
            vote_points: value.vote_points,
            account_type: value.account_type,
            created_at: value.created_at,
        }
    }
}

impl User {

}
