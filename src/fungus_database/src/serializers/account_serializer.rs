use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct AccountSerializer {
    pub id: i32,                   // auto incr postgresql SERIAL
    pub world_id: i16,             // default is 0
    pub character_slots: i16,      // default is 3
    pub created_at: NaiveDateTime, // default is now()

    // Foreign keys
    pub user_id: i32,              // users table foreign key
    pub trunk_id: i32,                 // trunks table foreign key

}