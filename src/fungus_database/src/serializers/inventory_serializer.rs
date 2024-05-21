use sqlx::{Postgres, Transaction};
use fungus_utils::constants::game_constants::{DEFAULT_INVENTORY_SIZE, MAX_INVENTORY_SIZE};
use fungus_utils::enums::inv_type::InvType;

pub struct InventorySerializer {
    pub id: i64,
    pub slots: i16,
    pub inv_type: InvType
}

impl InventorySerializer {
    pub async fn create_query(tx: &mut Transaction<'_, Postgres>, slots: i16, inv_type: InvType) -> Result<InventorySerializer, sqlx::Error>{
        sqlx::query_as!(
            InventorySerializer,
            "INSERT INTO inventories (slots, inv_type) VALUES ($1, $2) RETURNING *",
            slots, inv_type as i16
        ).fetch_one(&mut *tx).await
    }

    // TODO add character details.
    pub async fn create_inventory(tx: &mut Transaction<'_, Postgres>) -> Result<(), sqlx::Error> {
        let equipped_inventory = InventorySerializer::create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::None // Yeah lol
        ).await?;

        let equip_inventory = InventorySerializer::create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Equip
        ).await?;

        let consume_inventory = InventorySerializer::create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Consume
        ).await?;


        let install_inventory = InventorySerializer::create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Install
        ).await?;

        let etc_inventory = InventorySerializer::create_query(
            &mut *tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Etc
        ).await?;

        let cash_inventory = InventorySerializer::create_query(
            &mut *tx,
            MAX_INVENTORY_SIZE,
            InvType::Cash
        ).await?;
        Ok(())
    }
}