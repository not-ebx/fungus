use fungus_database::serializers::equipment_serializer::EquipmentSerializer;
use fungus_utils::enums::item_type::ItemType;
use crate::entities::item::Item;

#[derive(Clone)]
pub struct Equipment {
    pub item: Item,
    //pub item_id: i64, // items table foreign key and key for this table too, since there's no duplicates.

    pub attribute: i16,
    pub attack_speed: i32,

    // Stat Requirement
    pub req_str: i32,
    pub req_dex: i32,
    pub req_int: i32,
    pub req_luk: i32,
    pub req_level: i32,
    pub req_pop: i32,
    pub req_job: i32,

    // Stat increase
    pub inc_hp: i16,
    pub inc_mp: i16,
    pub inc_str: i32,
    pub inc_dex: i32,
    pub inc_int: i32,
    pub inc_luk: i32,
    pub inc_accuracy: i16,
    pub inc_craft: i16, // AKA hands
    pub inc_evasion: i16, // AKA avoid
    pub inc_jump: i16,
    pub inc_speed: i16,
    pub inc_mad: i16, // AD is Attack
    pub inc_mdd: i16, // DD is defense
    pub inc_pad: i16,
    pub inc_pdd: i16,
    pub inc_ied: i16, //aka MDR
    pub inc_total_damage: i16, // AKA DAMr
    pub inc_pvp_damage: i16,
    pub inc_reduce_req: i16, // Reduce level req
    pub inc_boss_damage_range: i16, // known as bdR in wz

    // Item upgrade details
    pub total_upgrade_count: i16,
    pub current_upgrade_count: i16, // cuc
    pub enchant_count: i16, //chuc
    pub inc_upgrade_count: i16, // Hammers used, aka iuc

    // Misc
    pub charm_exp: i32,
    pub exp: i16,
    pub item_level: i16,
    pub durability: i16,
    pub durability_max: i16,
    pub price: i32,
    pub serial_number: i64,
    pub i_slot: String, // Varchar, emtpy as default
    pub v_slot: String, // Varchar, empty ad default
    pub ps_enchant: i16, // Final Strike
    pub set_id: i32, // Equipment set id. In-game data, too.
    pub android: i32,
    pub android_grade: i32,

    // Details. Bools are false by default, unless stated.
    pub is_trade_blocked: bool,
    pub is_unique: bool, // In-game uniqueness value, not related to database
    pub is_potable: bool, // is_potable is TRUE by default
    pub is_expired_on_logout: bool,
    pub is_boss_reward: bool,
    pub has_fixed_potential: bool,
    pub is_sellable: bool, // is_sellable is TRUE by default
    pub is_sokable: bool, // Can use Siccors of Karma (sok)
    pub is_superior_equip: bool,

}


impl Into<EquipmentSerializer> for Equipment {
    fn into(self) -> EquipmentSerializer {
        EquipmentSerializer {
            item_id: self.item.id,
            attribute: self.attribute,
            attack_speed: self.attack_speed,
            req_str: self.req_str,
            req_dex: self.req_dex,
            req_int: self.req_int,
            req_luk: self.req_luk,
            req_level: self.req_level,
            req_pop: self.req_pop,
            req_job: self.req_job,
            inc_hp: self.inc_hp,
            inc_mp: self.inc_mp,
            inc_str: self.inc_str,
            inc_dex: self.inc_dex,
            inc_int: self.inc_int,
            inc_luk: self.inc_luk,
            inc_accuracy: self.inc_accuracy,
            inc_craft: self.inc_craft,
            inc_evasion: self.inc_evasion,
            inc_jump: self.inc_jump,
            inc_speed: self.inc_speed,
            inc_mad: self.inc_mad,
            inc_mdd: self.inc_mdd,
            inc_pad: self.inc_pad,
            inc_pdd: self.inc_pdd,
            inc_ied: self.inc_ied,
            inc_total_damage: self.inc_total_damage,
            inc_pvp_damage: self.inc_pvp_damage,
            inc_reduce_req: self.inc_reduce_req,
            inc_boss_damage_range: self.inc_boss_damage_range,
            total_upgrade_count: self.total_upgrade_count,
            current_upgrade_count: self.current_upgrade_count,
            enchant_count: self.enchant_count,
            inc_upgrade_count: self.inc_upgrade_count,
            charm_exp: self.charm_exp,
            exp: self.exp,
            item_level: self.item_level,
            durability: self.durability,
            durability_max: self.durability_max,
            price: self.price,
            serial_number: self.serial_number,
            i_slot: self.i_slot.clone(),
            v_slot: self.v_slot.clone(),
            ps_enchant: self.ps_enchant,
            set_id: self.set_id,
            android: self.android,
            android_grade: self.android_grade,
            is_trade_blocked: self.is_trade_blocked,
            is_unique: self.is_unique,
            is_potable: self.is_potable,
            is_expired_on_logout: self.is_expired_on_logout,
            is_boss_reward: self.is_boss_reward,
            has_fixed_potential: self.has_fixed_potential,
            is_sellable: self.is_sellable,
            is_sokable: self.is_sokable,
            is_superior_equip: self.is_superior_equip,
        }
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Equipment {
            item: Default::default(),
            attribute: 0,
            attack_speed: 0,
            req_str: 0,
            req_dex: 0,
            req_int: 0,
            req_luk: 0,
            req_level: 0,
            req_pop: 0,
            req_job: 0,
            inc_hp: 0,
            inc_mp: 0,
            inc_str: 0,
            inc_dex: 0,
            inc_int: 0,
            inc_luk: 0,
            inc_accuracy: 0,
            inc_craft: 0,
            inc_evasion: 0,
            inc_jump: 0,
            inc_speed: 0,
            inc_mad: 0,
            inc_mdd: 0,
            inc_pad: 0,
            inc_pdd: 0,
            inc_ied: 0,
            inc_total_damage: 0,
            inc_pvp_damage: 0,
            inc_reduce_req: 0,
            inc_boss_damage_range: 0,
            total_upgrade_count: 0,
            current_upgrade_count: 0,
            enchant_count: 0,
            inc_upgrade_count: 0,
            charm_exp: 0,
            exp: 0,
            item_level: 0,
            durability: 0,
            durability_max: 0,
            price: 0,
            serial_number: 0,
            i_slot: "".to_string(),
            v_slot: "".to_string(),
            ps_enchant: 0,
            set_id: 0,
            android: 0,
            android_grade: 0,
            is_trade_blocked: false,
            is_unique: false,
            is_potable: true,
            is_expired_on_logout: false,
            is_boss_reward: false,
            has_fixed_potential: false,
            is_sellable: true,
            is_sokable: false,
            is_superior_equip: false,
        }
    }
}


impl Equipment {
    //In-memory only!
    pub fn new_default(item_id: i32) -> Self {
        let item = Item::new_default(item_id, ItemType::Equip);
        let mut equipment = Equipment::default();
        equipment.item = item;
        equipment
    }

}