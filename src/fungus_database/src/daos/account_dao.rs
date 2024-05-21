use sqlx::{Error, PgPool, Postgres, Transaction};
use crate::serializers::account_serializer::AccountSerializer;
use crate::serializers::trunk_serializer::TrunkSerializer;

pub struct AccountDAO;

impl AccountDAO {
    pub async fn create_account(&self, tx: &mut Transaction<'_, Postgres>, user_id: i32, world_id: i16, trunk: &TrunkSerializer) -> Result<AccountSerializer, Error> {
        sqlx::query_as!(
            AccountSerializer,
            "INSERT INTO accounts (world_id, character_slots, user_id, trunk_id) VALUES ($1, $2, $3, $4) RETURNING *",
            world_id, 3, user_id, trunk.id
        ).fetch_one(tx).await
    }

    pub async fn get_user_account(&self, pool: &PgPool, user_id: i32, world_id: i16) -> Result<AccountSerializer, Error> {
        sqlx::query_as!(
            AccountSerializer,
            "SELECT * FROM accounts WHERE user_id = $1 AND world_id = $2",
            user_id, world_id
        ).fetch_one(pool).await
    }
}