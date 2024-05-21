use sqlx::{Error, Postgres, Transaction};
use fungus_database::serializers::equipment_serializer::EquipmentSerializer;
use fungus_database::serializers::item_serializer::ItemSerializer;

pub trait CreateGameItem<T> {
    async fn create_item(tx: &mut Transaction<'_, Postgres>, item: &mut T) -> Result<(), sqlx::Error>;
}


pub struct ItemService {

}

impl CreateGameItem<ItemSerializer> for ItemService {
    async fn create_item(tx: &mut Transaction<'_, Postgres>, item: &mut ItemSerializer) -> Result<(), sqlx::Error>{
        let item_id = sqlx::query_scalar!(
            "INSERT INTO items (\
            bag_index, cash_serial_number, inv_type, item_type, is_cash, item_id, owner, quantity, inventory_id, trunk_id, expires_at\
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING id",
            item.bag_index, item.cash_serial_number, item.inv_type as i16, item.item_type as i16, item.is_cash, item.item_id, item.owner, item.quantity, item.inventory_id, item.trunk_id, item.expires_at
        ).fetch_one(&mut *tx).await?;

        item.id = item_id;

        Ok(())
    }
}

impl CreateGameItem<EquipmentSerializer> for ItemService {
    async fn create_item(tx: &mut Transaction<'_, Postgres>, item: &mut EquipmentSerializer) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO equipments (\
                item_id,\
                attribute,\
                attack_speed,\
                req_str,\
                req_dex,\
                req_int,\
                req_luk,\
                req_level,\
                req_pop,\
                req_job,\
                inc_hp,\
                inc_mp,\
                inc_str,\
                inc_dex,\
                inc_int,\
                inc_luk,\
                inc_accuracy,\
                inc_craft,\
                inc_evasion,\
                inc_jump,\
                inc_speed,\
                inc_mad,\
                inc_mdd,\
                inc_pad,\
                inc_pdd,\
                inc_ied,\
                inc_total_damage,\
                inc_pvp_damage,\
                inc_reduce_req,\
                inc_boss_damage_range,\
                total_upgrade_count,\
                current_upgrade_count,\
                enchant_count,\
                inc_upgrade_count,\
                charm_exp,\
                exp,\
                item_level,\
                durability,\
                durability_max,\
                price,\
                serial_number,\
                i_slot,\
                v_slot,\
                ps_enchant,\
                set_id,\
                android,\
                android_grade,\
                is_trade_blocked,\
                is_unique,\
                is_potable,\
                is_expired_on_logout,\
                is_boss_reward,\
                has_fixed_potential,\
                is_sellable,\
                is_sokable,\
                is_superior_equip\
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, $53, $54, $55, $56)",
            item.item_id, item.attribute, item.attack_speed, item.req_str, item.req_dex, item.req_int, item.req_luk, item.req_level, item.req_pop, item.req_job, item.inc_hp, item.inc_mp, item.inc_str, item.inc_dex, item.inc_int, item.inc_luk, item.inc_accuracy, item.inc_craft, item.inc_evasion, item.inc_jump, item.inc_speed, item.inc_mad, item.inc_mdd, item.inc_pad, item.inc_pdd, item.inc_ied, item.inc_total_damage, item.inc_pvp_damage, item.inc_reduce_req, item.inc_boss_damage_range, item.total_upgrade_count, item.current_upgrade_count, item.enchant_count, item.inc_upgrade_count, item.charm_exp, item.exp, item.item_level, item.durability, item.durability_max, item.price, item.serial_number, item.i_slot, item.v_slot, item.ps_enchant, item.set_id, item.android, item.android_grade, item.is_trade_blocked, item.is_unique, item.is_potable, item.is_expired_on_logout, item.is_boss_reward, item.has_fixed_potential, item.is_sellable, item.is_sokable, item.is_superior_equip
        ).execute(&mut *tx).await?;

        Ok(())
    }
}

impl ItemService {
    pub fn new() -> Self {
        ItemService {}
    }
}