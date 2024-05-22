use sqlx::{Error, Postgres, Transaction};
use crate::serializers::avatar_look_serializer::AvatarLookSerializer;

pub struct AvatarLookDAO;

impl AvatarLookDAO {
    pub async fn create_query(&self, tx: &mut Transaction<'_, Postgres>, face: i32, hair: i32, skin: i32) -> Result<AvatarLookSerializer, Error> {
        sqlx::query_as!(
            AvatarLookSerializer,
            "INSERT INTO avatar_looks (face, hair, skin, elf_ear, ears, demon_slayer_mark) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            face, hair, skin, false, 0, 0
        ).fetch_one(&mut **tx).await
    }
}