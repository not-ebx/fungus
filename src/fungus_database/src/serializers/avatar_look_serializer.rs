use sqlx::{Error, Postgres, Transaction};

pub struct AvatarLookSerializer {
    pub id: i32,
    pub face: i32,
    pub hair: i32,
    pub skin: i32,
    pub elf_ear: bool,
    pub ears: i32,

    // Face Accessories
    pub demon_slayer_mark: i32,
}

impl AvatarLookSerializer {
    // TODO extend for mercedes and DS. eventually.
    pub async fn create_query(tx: &mut Transaction<'_, Postgres>, face: i32, hair: i32, skin: i32) -> Result<AvatarLookSerializer, Error> {
        sqlx::query_as!(
            AvatarLookSerializer,
            "INSERT INTO avatar_looks (face, hair, skin, elf_ear, ears, demon_slayer_mark) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            face, hair, skin, false, 0, 0
        ).fetch_one(&mut *tx).await
    }
}