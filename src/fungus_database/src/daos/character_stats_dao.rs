use sqlx::{Error, PgPool, Postgres, Transaction};
use crate::serializers::character_stats_serializer::CharacterStatsSerializer;

pub struct CharacterStatsDAO;

impl CharacterStatsDAO {
    pub async fn create_query(&self, tx: &mut Transaction<'_, Postgres>, name: &str, gender: u8, job: i32, sub_job: i16) -> Result<CharacterStatsSerializer, Error> {
        sqlx::query_as!(
            CharacterStatsSerializer,
            "INSERT INTO character_stats (name, gender, job, sub_job, str, dex, int, luk, hp, max_hp, mp, max_mp) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *",
            name.to_string(), gender as i16, job, sub_job as i32, 12, 5, 4, 4, 50, 50, 5, 5
        ).fetch_one(&mut **tx).await
    }

    pub async fn is_name_taken(&self, pool: &PgPool, name: &str) -> bool {
        let val = sqlx::query_scalar!(
            "SELECT id FROM character_stats WHERE name = $1",
            name
        ).fetch_one(pool).await;

        if let Err(_) = val {
            false
        } else {
            true
        }
    }
}