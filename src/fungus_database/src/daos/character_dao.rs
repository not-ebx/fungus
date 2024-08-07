use sqlx::{Column, Error, PgPool, Postgres, Row, Transaction};
use crate::serializers::avatar_look_serializer::AvatarLookSerializer;
use crate::serializers::character_serializer::CharacterSerializer;
use crate::serializers::character_select_serializer::CharacterSelectSerializer;
use crate::serializers::character_stats_serializer::CharacterStatsSerializer;

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

    pub async fn get_characters_for_login_screen(&self, pool: &PgPool, account_id: i32) -> Vec<CharacterSelectSerializer> {
        let chara_rows = sqlx::query(
            r#"
            SELECT character.*, avatar_look.*, character_stats.*
            FROM characters character
            JOIN avatar_looks avatar_look ON character.avatar_look_id = avatar_look.id
            JOIN character_stats character_stats ON character.character_stats_id = character_stats.id
            WHERE character.account_id = $1
            "#,
        ).bind(account_id).fetch_all(pool).await.unwrap();

        let mut charas: Vec<CharacterSelectSerializer> = chara_rows.iter().map(|row| {
            let character= CharacterSerializer::try_from(row).ok();
            let character_stats= CharacterStatsSerializer::try_from(row).ok();
            let avatar_look = AvatarLookSerializer::try_from(row).ok();
            if character.is_none() || character_stats.is_none() || avatar_look.is_none() {
                return None
            }
            Some(CharacterSelectSerializer{
                character: character.unwrap(),
                character_stats: character_stats.unwrap(),
                avatar_look: avatar_look.unwrap()
            })
        }).filter_map(|x| x).collect();

        charas
    }

    pub async fn get_character_by_id(&self, pool: &PgPool, character_id: i32) -> Result<CharacterSelectSerializer, Error> {
        let row = sqlx::query(
            r#"
            SELECT character.*, avatar_look.*, character_stats.*
            FROM characters character
            JOIN avatar_looks avatar_look ON character.avatar_look_id = avatar_look.id
            JOIN character_stats character_stats ON character.character_stats_id = character_stats.id
            WHERE character.id = $1
            "#,
        ).bind(character_id).fetch_one(pool).await.unwrap();

        let character= CharacterSerializer::try_from(&row).ok();
        let character_stats= CharacterStatsSerializer::try_from(&row).ok();
        let avatar_look = AvatarLookSerializer::try_from(&row).ok();
        if character.is_none() || character_stats.is_none() || avatar_look.is_none() {
            return Err(Error::RowNotFound)
        }

        Ok(CharacterSelectSerializer{
            character: character.unwrap(),
            character_stats: character_stats.unwrap(),
            avatar_look: avatar_look.unwrap()
        })
    }

}