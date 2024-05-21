use chrono::NaiveDateTime;

pub struct CharacterLoginSerializer {
    id: i64,
    character_id: i32,
    login_date: NaiveDateTime, // Default at now()
    logout_date: NaiveDateTime, // Default at now(), in case of a crash immediately
    ip: String,
    machine_id: String,
}