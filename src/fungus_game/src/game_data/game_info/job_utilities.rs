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

