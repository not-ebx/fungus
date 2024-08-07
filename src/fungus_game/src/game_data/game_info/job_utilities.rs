use strum_macros::{EnumIter, FromRepr};
use crate::game_data::game_info::job_races::JobRace;

#[derive(Debug, Eq, PartialEq, EnumIter, FromRepr)]
#[repr(i32)]
pub enum JobIdentifier {
    Beginner = 0,

    Warrior = 100,
    Fighter = 110,
    Crusader = 111,
    Hero = 112,
    Page = 120,
    WhiteKnight = 121,
    Paladin = 122,
    Spearman = 130,
    DragonKnight = 131,
    DarkKnight = 132,

    Magician = 200,
    FpWizard = 210,
    FpMage = 211,
    FpArchmage = 212,
    IlWizard = 220,
    IlMage = 221,
    IlArchmage = 222,
    Cleric = 230,
    Priest = 231,
    Bishop = 232,

    Bowman = 300,
    Hunter = 310,
    Ranger = 311,
    Bowmaster = 312,
    Crossbowman = 320,
    Sniper = 321,
    Marksman = 322,

    Thief = 400,
    Assassin = 410,
    Hermit = 411,
    NightLord = 412,
    Bandit = 420,
    ChiefBandit = 421,
    Shadower = 422,

    BladeRecruit = 430,
    BladeAcolyte = 431,
    BladeSpecialist = 432,
    BladeLord = 433,
    BladeMaster = 434,

    Pirate = 500,
    PirateCannoneer = 501,
    Jett1 = 508,
    Brawler = 510,
    Marauder = 511,
    Buccaneer = 512,
    Gunslinger = 520,
    Outlaw = 521,
    Corsair = 522,

    Cannoneer = 530,
    CannonBlaster = 531,
    CannonMaster = 532,

    Manager = 800,
    Gm = 900,
    SuperGm = 910,

    Noblesse = 1000,
    DawnWarrior1 = 1100,
    DawnWarrior2 = 1110,
    DawnWarrior3 = 1111,
    DawnWarrior4 = 1112,
    BlazeWizard1 = 1200,
    BlazeWizard2 = 1210,
    BlazeWizard3 = 1211,
    BlazeWizard4 = 1212,
    WindArcher1 = 1300,
    WindArcher2 = 1310,
    WindArcher3 = 1311,
    WindArcher4 = 1312,
    NightWalker1 = 1400,
    NightWalker2 = 1410,
    NightWalker3 = 1411,
    NightWalker4 = 1412,
    ThunderBreaker1 = 1500,
    ThunderBreaker2 = 1510,
    ThunderBreaker3 = 1511,
    ThunderBreaker4 = 1512,

    // Beginner Heros
    Legend = 2000,
    EvanBeginner = 2001,
    Mercedes = 2002,
    Phantom = 2003,

    Aran1 = 2100,
    Aran2 = 2110,
    Aran3 = 2111,
    Aran4 = 2112,

    Evan1 = 2210,
    Evan2 = 2212,
    Evan3 = 2214,
    Evan4 = 2218,

    Mercedes1 = 2300,
    Mercedes2 = 2310,
    Mercedes3 = 2311,
    Mercedes4 = 2312,

    Phantom1 = 2400,
    Phantom2 = 2410,
    Phantom3 = 2411,
    Phantom4 = 2412,

    Citizen = 3000,
    DemonSlayer = 3001,
    DemonSlayer1 = 3100,
    DemonSlayer2 = 3110,
    DemonSlayer3 = 3111,
    DemonSlayer4 = 3112,
    BattleMage1 = 3200,
    BattleMage2 = 3210,
    BattleMage3 = 3211,
    BattleMage4 = 3212,
    WildHunter1 = 3300,
    WildHunter2 = 3310,
    WildHunter3 = 3311,
    WildHunter4 = 3312,
    Mechanic1 = 3500,
    Mechanic2 = 3510,
    Mechanic3 = 3511,
    Mechanic4 = 3512,
}

impl From<i32> for JobIdentifier {
    fn from(value: i32) -> Self {
        JobIdentifier::from_repr(value).unwrap_or(JobIdentifier::Beginner)
    }
}

impl From<JobRace> for JobIdentifier {
    fn from(value: JobRace) -> Self {
        match value {
            JobRace::Resistance => JobIdentifier::Citizen,
            JobRace::Explorer => JobIdentifier::Beginner,
            JobRace::Cygnus => JobIdentifier::Noblesse,
            JobRace::Aran => JobIdentifier::Legend,
            JobRace::Evan => JobIdentifier::EvanBeginner,
            JobRace::Mercedes => JobIdentifier::Mercedes,
            JobRace::Demon => JobIdentifier::DemonSlayer,
            JobRace::Phantom => JobIdentifier::Phantom,
            JobRace::DualBlade => JobIdentifier::Beginner,
        }
    }
}

pub fn is_hero(job: i16) -> bool {
    job / 10 == 11
}

pub fn is_paladin(job: i16) -> bool {
    job / 10 == 12
}

pub fn is_dark_knight(job: i16) -> bool {
    job / 10 == 13
}

pub fn is_fire_poison(job: i16) -> bool {
    job / 10 == 21
}

pub fn is_ice_lightning(job: i16) -> bool {
    job / 10 == 22
}

pub fn is_bishop(job: i16) -> bool {
    job / 10 == 23
}

pub fn is_bowmaster(job: i16) -> bool {
    job / 10 == 31
}

pub fn is_marksman(job: i16) -> bool {
    job / 10 == 32
}

pub fn is_night_lord(job: i16) -> bool {
    job / 10 == 41
}

pub fn is_shadower(job: i16) -> bool {
    job / 10 == 42
}

pub fn is_dual_blade(job: i16) -> bool {
    job / 10 == 43
}

pub fn is_buccaneer(job: i16) -> bool {
    job / 10 == 51
}

pub fn is_corsair(job: i16) -> bool {
    job / 10 == 52
}

pub fn is_battle_mage(job: i16) -> bool {
    job / 100 == 32
}

pub fn is_wild_hunter(job: i16) -> bool {
    job / 100 == 33
}

pub fn is_mechanic(job: i16) -> bool {
    job / 100 == 35
}

pub fn get_damage_constant(job: i16) -> f64 {
    if job > 222 {
        if job > 1200 {
            if job >= 1210 && job <= 1212 {
                return 0.2;
            }
        } else if job == 1200 || (job >= 230 && job <= 232) {
            return 0.2;
        }
        return 0.0;
    }
    if job < 220 {
        match job {
            110 | 111 | 112 => 0.1,
            200 | 210 | 211 | 212 => 0.2,
            _ => 0.0,
        }
    } else {
        0.2
    }
}

pub fn get_job_category(job: i16) -> i32 {
    match job / 100 {
        27 | 140 | 142 => 2,
        36 => 4,
        37 => 1,
        _ => (job % 1000 / 100) as i32,
    }
}

pub fn is_adventurer_warrior(job_id: i16) -> bool {
    job_id == 100 || is_hero(job_id) || is_paladin(job_id) || is_dark_knight(job_id)
}

pub fn is_adventurer_mage(job_id: i16) -> bool {
    job_id == 200 || is_fire_poison(job_id) || is_ice_lightning(job_id) || is_bishop(job_id)
}

pub fn is_adventurer_archer(job_id: i16) -> bool {
    job_id == 300 || is_bowmaster(job_id) || is_marksman(job_id)
}

pub fn is_adventurer_thief(job_id: i16) -> bool {
    job_id == 400 || is_night_lord(job_id) || is_shadower(job_id) || is_dual_blade(job_id)
}

pub fn is_adventurer_pirate(job_id: i16) -> bool {
    job_id == 500 || is_buccaneer(job_id) || is_corsair(job_id)
}

pub fn is_adventurer(job_id: i16) -> bool {
    job_id == 0 || is_adventurer_warrior(job_id) || is_adventurer_mage(job_id) || is_adventurer_archer(job_id) ||
        is_adventurer_thief(job_id) || is_adventurer_pirate(job_id)
}

pub fn is_cygnus_knight(job_id: i16) -> bool {
    job_id / 1000 == 1
}

pub fn is_resistance(job_id: i16) -> bool {
    job_id / 1000 == 3
}

pub fn is_leader(job_id: i16) -> bool {
    job_id / 1000 == 5
}

pub fn is_hidden(job_id: i16) -> bool {
    job_id / 100 == 25 || job_id == 2005
}

pub fn is_extend_sp_job(job_id: i16) -> bool {
    is_resistance(job_id)
}

pub fn is_beginner_job(job_id: i16) -> bool {
    match job_id {
        8001 | 13000 | 14000 | 6000 | 6001 | 5000 | 4001 | 4002 | 3001 | 3002 | 2001 | 2002 | 2003 | 2004 | 2005 => true,
        _ => job_id % 1000 == 0 || job_id / 100 == 8000,
    }
}

