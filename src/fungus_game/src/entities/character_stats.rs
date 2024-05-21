use chrono::NaiveDateTime;

pub struct CharacterStats {
    // Character Details
    id: i32,
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
