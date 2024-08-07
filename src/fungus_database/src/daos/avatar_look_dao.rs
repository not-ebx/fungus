use sqlx::{Error, Postgres, Transaction};
use crate::serializers::avatar_look_serializer::AvatarLookSerializer;

pub struct AvatarLookDAO;

impl AvatarLookDAO {
    pub async fn create_query(&self, tx: &mut Transaction<'_, Postgres>, face: i32, hair: i32, skin: i32, gender: i16, job: i32) -> Result<AvatarLookSerializer, Error> {
        sqlx::query_as!(
            AvatarLookSerializer,
            "INSERT INTO avatar_looks (face, hair, skin, job, gender, elf_ear, ears, demon_slayer_mark) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
            face, hair, skin, job, gender, false, 0, 0
        ).fetch_one(&mut **tx).await
    }

    pub async fn update(&self, tx: &mut Transaction<'_, Postgres>, avatar_look: AvatarLookSerializer) -> Result<(), Error> {
        let _ = sqlx::query!(
           r#"
            UPDATE avatar_looks
            SET face = $1,
                hair = $2,
                skin = $3,
                job = $4,
                gender = $5,
                weapon_id = $6,
                sub_weapon_id = $7,
                weapon_sticker_id = $8,
                elf_ear = $9,
                ears = $10,
                demon_slayer_mark = $11
            WHERE id = $12
            "#,
            avatar_look.face,
            avatar_look.hair,
            avatar_look.skin,
            avatar_look.job,
            avatar_look.gender,
            avatar_look.weapon_id,
            avatar_look.sub_weapon_id,
            avatar_look.weapon_sticker_id,
            avatar_look.elf_ear,
            avatar_look.ears,
            avatar_look.demon_slayer_mark,
            avatar_look.id
        ).execute(&mut **tx).await?;

        Ok(())
    }
}