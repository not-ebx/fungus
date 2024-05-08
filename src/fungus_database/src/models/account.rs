use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,                   // auto incr postgresql SERIAL
    pub world_id: i16,             // default is 0
    pub storage_mesos: i64,        // default is 0
    pub character_slots: i16,      // default is 3
    pub created_at: NaiveDateTime, // default is now()
    pub user_id: i32,              // users table foreign key
}
