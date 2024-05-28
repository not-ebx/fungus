use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, Row};
use sqlx::database::HasValueRef;
use sqlx::postgres::PgRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
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
    pub map_id: i64,
    pub portal: i32,

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

impl TryFrom<&PgRow> for CharacterStatsSerializer {
    type Error = sqlx::Error;

    fn try_from(row: &PgRow) -> Result<Self, Self::Error> {
        Ok(CharacterStatsSerializer{
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            gender: row.try_get("gender")?,
            job: row.try_get("job")?,
            sub_job: row.try_get("sub_job")?,
            level: row.try_get("level")?,
            exp: row.try_get("exp")?,
            mesos: row.try_get("mesos")?,
            ap: row.try_get("ap")?,
            sp: row.try_get("sp")?,
            map_id: row.try_get("map_id")?,
            portal: row.try_get("portal")?,
            str: row.try_get("str")?,
            dex: row.try_get("dex")?,
            int: row.try_get("int")?,
            luk: row.try_get("luk")?,
            hp: row.try_get("hp")?,
            max_hp: row.try_get("max_hp")?,
            mp: row.try_get("mp")?,
            max_mp: row.try_get("max_mp")?,
            pop: row.try_get("pop")?,
            willpower_level: row.try_get("willpower_level")?,
            willpower_exp: row.try_get("willpower_exp")?,
            charm_level: row.try_get("charm_level")?,
            charm_exp: row.try_get("charm_exp")?,
            insight_level: row.try_get("insight_level")?,
            insight_exp: row.try_get("insight_exp")?,
            ambition_level: row.try_get("ambition_level")?,
            ambition_exp: row.try_get("ambition_exp")?,
            empathy_level: row.try_get("empathy_level")?,
            empathy_exp: row.try_get("empathy_exp")?,
            diligence_level: row.try_get("diligence_level")?,
            diligence_exp: row.try_get("diligence_exp")?,
            craft_level: row.try_get("craft_level")?,
            craft_exp: row.try_get("craft_exp")?,
            fatigue: row.try_get("fatigue")?,
            honor_exp: row.try_get("honor_exp")?,
            pvp_exp: row.try_get("pvp_exp")?,
            pvp_grade: row.try_get("pvp_grade")?,
            pvp_mode_level: row.try_get("pvp_mode_level")?,
            pvp_mode_type: row.try_get("pvp_mode_type")?,
            pvp_point: row.try_get("pvp_point")?,
            pop_available_at: row.try_get("pop_available_at")?,
            fatigue_updated_at: row.try_get("fatigue_updated_at")?,
        })
    }
}