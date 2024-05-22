use sqlx::{Error, Postgres, Transaction};
use crate::serializers::trunk_serializer::TrunkSerializer;

pub struct TrunkDAO;

impl TrunkDAO {
    pub async fn create_query(&self, tx: &mut Transaction<'_, Postgres>) -> Result<TrunkSerializer, Error> {
        sqlx::query_as!(
            TrunkSerializer,
            "INSERT INTO trunks (slots, mesos) VALUES ($1, $2) RETURNING *",
            4, 0
        ).fetch_one(&mut **tx).await
    }
}