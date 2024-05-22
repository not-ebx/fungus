use sqlx::{Postgres, Transaction};
use fungus_utils::constants::game_constants::{DEFAULT_INVENTORY_SIZE, MAX_INVENTORY_SIZE};
use fungus_utils::enums::inv_type::InvType;
use crate::serializers::inventory_serializer::InventorySerializer;

pub struct InventoryDAO;

impl InventoryDAO {
    pub async fn create_query(&self, tx: &mut Transaction<'_, Postgres>, slots: i16, inv_type: InvType) -> Result<InventorySerializer, sqlx::Error>{
        let i16_inv_type: i16 = inv_type.into();
        sqlx::query_as!(
            InventorySerializer,
            "INSERT INTO inventories (slots, inv_type) VALUES ($1, $2) RETURNING *",
            slots, i16_inv_type
        ).fetch_one(&mut **tx).await
    }

    // TODO add character details.
    pub async fn create_inventory(&self, tx: &mut Transaction<'_, Postgres>) -> Result<Vec<i64>, sqlx::Error> {
        let equipped_inventory = self.create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::None // Yeah lol
        ).await?;

        let equip_inventory = self.create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Equip
        ).await?;

        let consume_inventory = self.create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Consume
        ).await?;


        let install_inventory = self.create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Install
        ).await?;

        let etc_inventory = self.create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Etc
        ).await?;

        let cash_inventory = self.create_query(
            &mut *tx,
            MAX_INVENTORY_SIZE,
            InvType::Cash
        ).await?;

        Ok(vec![
            equipped_inventory.id,
            equip_inventory.id,
            consume_inventory.id,
            install_inventory.id,
            etc_inventory.id,
            cash_inventory.id,
        ])
    }
}