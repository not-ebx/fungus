-- Inventories Table
CREATE TABLE inventories (
                             id BIGSERIAL PRIMARY KEY,
                             slots SMALLINT NOT NULL DEFAULT 52,
                             inv_type SMALLINT NOT NULL
);

-- Users Table
CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       username VARCHAR NOT NULL,
                       password VARCHAR NOT NULL,
                       birthday DATE NOT NULL,
                       gender SMALLINT NOT NULL DEFAULT 0,
                       nx_cash INTEGER NOT NULL DEFAULT 0,
                       maple_points INTEGER NOT NULL DEFAULT 0,
                       vote_points INTEGER NOT NULL DEFAULT 0,
                       account_type SMALLINT NOT NULL DEFAULT 0,
                       pic SMALLINT,
                       spw VARCHAR,
                       ban_expire_date TIMESTAMP,
                       ban_reason VARCHAR,
                       last_login TIMESTAMP,
                       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Accounts Table
CREATE TABLE accounts (
                          id SERIAL PRIMARY KEY,
                          world_id SMALLINT NOT NULL DEFAULT 0,
                          character_slots SMALLINT NOT NULL DEFAULT 3,
                          created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                          user_id INTEGER NOT NULL REFERENCES users(id),
                          trunk_id INTEGER NOT NULL
);

-- Trunks Table
CREATE TABLE trunks (
                        id SERIAL PRIMARY KEY,
                        slots SMALLINT NOT NULL DEFAULT 4,
                        mesos BIGINT NOT NULL DEFAULT 0
);

-- Character Stats Table
CREATE TABLE character_stats (
                                 id SERIAL PRIMARY KEY,
                                 name VARCHAR NOT NULL,
                                 gender SMALLINT NOT NULL DEFAULT 0,
                                 job INTEGER NOT NULL,
                                 sub_job INTEGER NOT NULL,
                                 level INTEGER NOT NULL DEFAULT 1,
                                 exp BIGINT NOT NULL DEFAULT 0,
                                 mesos BIGINT NOT NULL DEFAULT 0,
                                 ap INTEGER NOT NULL DEFAULT 0,
                                 sp INTEGER NOT NULL DEFAULT 0,
                                 str INTEGER NOT NULL DEFAULT 0,
                                 dex INTEGER NOT NULL DEFAULT 0,
                                 int INTEGER NOT NULL DEFAULT 0,
                                 luk INTEGER NOT NULL DEFAULT 0,
                                 hp INTEGER NOT NULL DEFAULT 0,
                                 max_hp INTEGER NOT NULL DEFAULT 0,
                                 mp INTEGER NOT NULL DEFAULT 0,
                                 max_mp INTEGER NOT NULL DEFAULT 0,
                                 pop INTEGER NOT NULL DEFAULT 0,
                                 willpower_level INTEGER NOT NULL DEFAULT 0,
                                 willpower_exp INTEGER NOT NULL DEFAULT 0,
                                 charm_level INTEGER NOT NULL DEFAULT 0,
                                 charm_exp INTEGER NOT NULL DEFAULT 0,
                                 insight_level INTEGER NOT NULL DEFAULT 0,
                                 insight_exp INTEGER NOT NULL DEFAULT 0,
                                 ambition_level INTEGER NOT NULL DEFAULT 0,
                                 ambition_exp INTEGER NOT NULL DEFAULT 0,
                                 empathy_level INTEGER NOT NULL DEFAULT 0,
                                 empathy_exp INTEGER NOT NULL DEFAULT 0,
                                 diligence_level INTEGER NOT NULL DEFAULT 0,
                                 diligence_exp INTEGER NOT NULL DEFAULT 0,
                                 craft_level INTEGER NOT NULL DEFAULT 0,
                                 craft_exp INTEGER NOT NULL DEFAULT 0,
                                 fatigue INTEGER NOT NULL DEFAULT 0,
                                 honor_exp INTEGER NOT NULL DEFAULT 0,
                                 pvp_exp INTEGER NOT NULL DEFAULT 0,
                                 pvp_grade INTEGER NOT NULL DEFAULT 0,
                                 pvp_mode_level INTEGER NOT NULL DEFAULT 0,
                                 pvp_mode_type INTEGER NOT NULL DEFAULT 0,
                                 pvp_point INTEGER NOT NULL DEFAULT 0,
                                 pop_available_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                 fatigue_updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Avatar Looks Table
CREATE TABLE avatar_looks (
                              id SERIAL PRIMARY KEY,
                              face INTEGER NOT NULL,
                              hair INTEGER NOT NULL,
                              skin INTEGER NOT NULL,
                              job INTEGER NOT NULL,
                              gender SMALLINT NOT NULL,
                              weapon_id INTEGER,
                              sub_weapon_id INTEGER,
                              weapon_sticker_id INTEGER,
                              elf_ear BOOLEAN NOT NULL DEFAULT false,
                              ears INTEGER NOT NULL,
                              demon_slayer_mark INTEGER NOT NULL
);

-- Characters Table
CREATE TABLE characters (
                            id SERIAL PRIMARY KEY,
                            deleted_at TIMESTAMP,
                            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                            account_id INTEGER NOT NULL REFERENCES accounts(id),
                            avatar_look_id INTEGER NOT NULL REFERENCES avatar_looks(id),
                            character_stats_id INTEGER NOT NULL REFERENCES character_stats(id),
                            last_login_id INTEGER,
                            equipped_inventory BIGINT NOT NULL REFERENCES inventories(id),
                            equip_inventory BIGINT NOT NULL REFERENCES inventories(id),
                            consume_inventory BIGINT NOT NULL REFERENCES inventories(id),
                            install_inventory BIGINT NOT NULL REFERENCES inventories(id),
                            etc_inventory BIGINT NOT NULL REFERENCES inventories(id),
                            cash_inventory BIGINT NOT NULL REFERENCES inventories(id)
);

-- Character Logins Table
CREATE TABLE character_logins (
                                  id BIGSERIAL PRIMARY KEY,
                                  character_id INTEGER NOT NULL REFERENCES characters(id),
                                  login_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                  logout_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                  ip VARCHAR NOT NULL,
                                  machine_id VARCHAR NOT NULL
);



-- Items Table
CREATE TABLE items (
                       id BIGSERIAL PRIMARY KEY,
                       bag_index INTEGER NOT NULL,
                       cash_serial_number BIGINT NOT NULL,
                       inv_type SMALLINT NOT NULL,
                       item_type SMALLINT NOT NULL,
                       is_cash BOOLEAN NOT NULL DEFAULT false,
                       item_id INTEGER NOT NULL,
                       owner VARCHAR NOT NULL,
                       quantity INTEGER NOT NULL DEFAULT 1,
                       inventory_id BIGINT REFERENCES inventories(id),
                       trunk_id INTEGER REFERENCES trunks(id),
                       expires_at TIMESTAMP
);

-- Equipments Table
CREATE TABLE equipments (
                            item_id BIGINT PRIMARY KEY REFERENCES items(id),
                            attribute SMALLINT NOT NULL,
                            attack_speed INTEGER NOT NULL,
                            req_str INTEGER NOT NULL DEFAULT 0,
                            req_dex INTEGER NOT NULL DEFAULT 0,
                            req_int INTEGER NOT NULL DEFAULT 0,
                            req_luk INTEGER NOT NULL DEFAULT 0,
                            req_level INTEGER NOT NULL DEFAULT 0,
                            req_pop INTEGER NOT NULL DEFAULT 0,
                            req_job INTEGER NOT NULL DEFAULT 0,
                            inc_hp SMALLINT NOT NULL DEFAULT 0,
                            inc_mp SMALLINT NOT NULL DEFAULT 0,
                            inc_str INTEGER NOT NULL DEFAULT 0,
                            inc_dex INTEGER NOT NULL DEFAULT 0,
                            inc_int INTEGER NOT NULL DEFAULT 0,
                            inc_luk INTEGER NOT NULL DEFAULT 0,
                            inc_accuracy SMALLINT NOT NULL DEFAULT 0,
                            inc_craft SMALLINT NOT NULL DEFAULT 0,
                            inc_evasion SMALLINT NOT NULL DEFAULT 0,
                            inc_jump SMALLINT NOT NULL DEFAULT 0,
                            inc_speed SMALLINT NOT NULL DEFAULT 0,
                            inc_mad SMALLINT NOT NULL DEFAULT 0,
                            inc_mdd SMALLINT NOT NULL DEFAULT 0,
                            inc_pad SMALLINT NOT NULL DEFAULT 0,
                            inc_pdd SMALLINT NOT NULL DEFAULT 0,
                            inc_ied SMALLINT NOT NULL DEFAULT 0,
                            inc_total_damage SMALLINT NOT NULL DEFAULT 0,
                            inc_pvp_damage SMALLINT NOT NULL DEFAULT 0,
                            inc_reduce_req SMALLINT NOT NULL DEFAULT 0,
                            inc_boss_damage_range SMALLINT NOT NULL DEFAULT 0,
                            total_upgrade_count SMALLINT NOT NULL DEFAULT 0,
                            current_upgrade_count SMALLINT NOT NULL DEFAULT 0,
                            enchant_count SMALLINT NOT NULL DEFAULT 0,
                            inc_upgrade_count SMALLINT NOT NULL DEFAULT 0,
                            charm_exp INTEGER NOT NULL DEFAULT 0,
                            exp SMALLINT NOT NULL DEFAULT 0,
                            item_level SMALLINT NOT NULL DEFAULT 0,
                            durability SMALLINT NOT NULL DEFAULT 0,
                            durability_max SMALLINT NOT NULL DEFAULT 0,
                            price INTEGER NOT NULL DEFAULT 0,
                            serial_number BIGINT NOT NULL,
                            i_slot VARCHAR NOT NULL DEFAULT '',
                            v_slot VARCHAR NOT NULL DEFAULT '',
                            ps_enchant SMALLINT NOT NULL DEFAULT 0,
                            set_id INTEGER NOT NULL,
                            android INTEGER NOT NULL DEFAULT 0,
                            android_grade INTEGER NOT NULL DEFAULT 0,
                            is_trade_blocked BOOLEAN NOT NULL DEFAULT false,
                            is_unique BOOLEAN NOT NULL DEFAULT false,
                            is_potable BOOLEAN NOT NULL DEFAULT true,
                            is_expired_on_logout BOOLEAN NOT NULL DEFAULT false,
                            is_boss_reward BOOLEAN NOT NULL DEFAULT false,
                            has_fixed_potential BOOLEAN NOT NULL DEFAULT false,
                            is_sellable BOOLEAN NOT NULL DEFAULT true,
                            is_sokable BOOLEAN NOT NULL DEFAULT false,
                            is_superior_equip BOOLEAN NOT NULL DEFAULT false
);

-- Sp Sets Table
CREATE TABLE sp_sets (
                         id BIGSERIAL PRIMARY KEY,
                         job_level SMALLINT NOT NULL,
                         sp SMALLINT NOT NULL DEFAULT 0,
                         character_stats_id INTEGER NOT NULL REFERENCES character_stats(id)
);
