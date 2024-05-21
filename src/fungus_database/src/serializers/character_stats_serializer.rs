use chrono::NaiveDateTime;
use sqlx::{Postgres, Transaction};

pub struct CharacterStatsSerializer {
    // Character Details
    pub id: i32,
    name: String,
    gender: i16,
    job: i32,
    sub_job: i32,
    level: i32, // Defaults at 1
    exp: i64,
    mesos: i64,
    ap: i32,
    sp: i32,
    //extendSP ==> Need a table for this :)

    // Character Stats
    str: i32,
    dex: i32,
    int: i32,
    luk: i32,
    hp: i32,
    max_hp: i32,
    mp: i32,
    max_mp: i32,
    pop: i32,

    // Profession & traits stats
    willpower_level: i32,
    willpower_exp: i32,
    charm_level: i32,
    charm_exp: i32,
    insight_level: i32,
    insight_exp: i32,
    ambition_level: i32,
    ambition_exp: i32,
    empathy_level: i32,
    empathy_exp: i32,
    diligence_level: i32,
    diligence_exp: i32,

    craft_level: i32,
    craft_exp: i32,
    fatigue: i32,
    honor_exp: i32,

    // Pvp stats
    pvp_exp: i32,
    pvp_grade: i32,
    pvp_mode_level: i32,
    pvp_mode_type: i32,
    pvp_point: i32,

    // Times
    pop_available_at: NaiveDateTime, //Defaults as current time
    fatigue_updated_at: NaiveDateTime, //Defaults as current time

}

impl CharacterStatsSerializer {
    pub async fn create_query(tx: &mut Transaction<'_, Postgres>, name: &str, gender: u8, job: i32, sub_job: i16) -> Result<CharacterStatsSerializer, sqlx::Error> {
        sqlx::query_as!(
            CharacterStatsSerializer,
            "INSERT INTO character_stats (name, gender, job, sub_job, str, dex, int, luk, hp, max_hp, mp, max_mp) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *",
            name.to_string(), gender as i16, job, sub_job as i32, 12, 5, 4, 4, 50, 50, 5, 5
        ).fetch_one(&mut *tx).await
    }
}