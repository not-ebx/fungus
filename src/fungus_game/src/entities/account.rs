use chrono::NaiveDateTime;
use sqlx::{Error, FromRow};
use fungus_database::serializers::account_serializer::AccountSerializer;
use crate::entities::character::Character;
use crate::entities::user::User;
use crate::entities::trunk::Trunk;

#[derive(FromRow)]
pub struct Account {
    pub id: i32,                   // auto incr postgresql SERIAL
    pub world_id: i16,             // default is 0
    //pub character_slots: i16,      // default is 3
    pub characters: Vec<i32>, // Just references to its ids.
    pub created_at: NaiveDateTime, // default is now()

    // Foreign keys
    pub user_id: i32,              // users table foreign key
    pub trunk_id: i32,                 // trunks table foreign key

}

impl From<AccountSerializer> for Account {
    fn from(value: AccountSerializer) -> Self {
        Account {
            id: value.id,
            world_id: value.world_id,
            characters: vec![0;value.character_slots as usize],
            created_at: Default::default(),
            user_id: value.user_id,
            trunk_id: value.trunk_id,
        }
    }
}

impl Account {

}