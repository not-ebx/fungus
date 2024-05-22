use sqlx::{Error, PgPool, Postgres, Transaction};
use crate::serializers::character_serializer::CharacterSerializer;

pub struct CharacterDAO;

impl CharacterDAO {
    pub async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        account_id: i32,
        character_stats_id: i32,
        avatar_look_id: i32,
        equipped_inventory_id: i64,
        equip_inventory_id: i64,
        consume_inventory_id: i64,
        install_inventory_id: i64,
        etc_inventory_id: i64,
        cash_inventory_id: i64
    ) -> Result<CharacterSerializer, Error> {
        sqlx::query_as!(
            CharacterSerializer,
            "INSERT INTO characters (\
                account_id,\
                character_stats_id,\
                avatar_look_id,\
                equipped_inventory,\
                equip_inventory,\
                consume_inventory,\
                install_inventory,\
                etc_inventory,\
                cash_inventory\
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
            account_id,
            character_stats_id,
            avatar_look_id,
            equipped_inventory_id,
            equip_inventory_id,
            consume_inventory_id,
            install_inventory_id,
            etc_inventory_id,
            cash_inventory_id,
        ).fetch_one(&mut **tx).await
    }

}