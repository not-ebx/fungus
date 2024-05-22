use chrono::NaiveDateTime;
use sqlx::{Postgres, Transaction};

pub struct CharacterStatsSerializer {
    // Character Details
    pub id: i32,
    pub name: String,
    pub gender: i16,
    pub job: i32,
    pub sub_job: i32,
    pub level: i32, // Defaults at 1
    pub exp: i64,
    pub mesos: i64,
    pub ap: i32,
    pub sp: i32,
    //extendSP ==> Need a table for this :)

    // Character Stats
    pub str: i32,
    pub dex: i32,
    pub int: i32,
    pub luk: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub pop: i32,

    // Profession & traits stats
    pub willpower_level: i32,
    pub willpower_exp: i32,
    pub charm_level: i32,
    pub charm_exp: i32,
    pub insight_level: i32,
    pub insight_exp: i32,
    pub ambition_level: i32,
    pub ambition_exp: i32,
    pub empathy_level: i32,
    pub empathy_exp: i32,
    pub diligence_level: i32,
    pub diligence_exp: i32,

    pub craft_level: i32,
    pub craft_exp: i32,
    pub fatigue: i32,
    pub honor_exp: i32,

    // Pvp stats
    pub pvp_exp: i32,
    pub pvp_grade: i32,
    pub pvp_mode_level: i32,
    pub pvp_mode_type: i32,
    pub pvp_point: i32,

    // Times
    pub pop_available_at: NaiveDateTime, //Defaults as current time
    pub fatigue_updated_at: NaiveDateTime, //Defaults as current time

}