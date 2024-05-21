use chrono::{NaiveDateTime};
use sqlx::{Error, Postgres};
use fungus_utils::constants::game_constants::{DEFAULT_INVENTORY_SIZE, MAX_INVENTORY_SIZE};
use fungus_utils::enums::inv_type::InvType;
use crate::database::get_db;
use fungus_game::::game_data::GAME_DATA;
use crate::models::avatar_look::AvatarLook;
use crate::models::equipment::Equipment;
use crate::models::inventory::Inventory;
use crate::models::item::Item;
use crate::serializers::avatar_look_serializer::AvatarLookSerializer;
use crate::serializers::character_serializer::CharacterSerializer;
use crate::serializers::character_stats_serializer::CharacterStatsSerializer;
use crate::serializers::equipment_serializer::EquipmentSerializer;
use crate::serializers::inventory_serializer::InventorySerializer;
use crate::serializers::item_serializer::ItemSerializer;
use fungus_services::item_service::{CreateGameItem, ItemService};

trait AddItemToInventory<T> {
    async fn add_item_to_inventory(&mut self, item: T) -> Result<(), sqlx::Error>;
}

pub struct Character {
    pub id: i32,
    // Times
    deleted_at: Option<NaiveDateTime>, // Soft delete!
    created_at: NaiveDateTime, // Defaults at now()

    equip_inventory: Inventory<Equipment>,
    use_inventory: Inventory<Item>,
    etc_inventory: Inventory<Item>,
    install_inventory: Inventory<Item>,
    cash_inventory: Inventory<Item>,
}

impl Character {
    pub fn new() {
    }

    pub async fn create_character(
        account_id: i32,
        name: &str,
        job: i32,
        sub_job: i16,
        gender: u8,
        skin: i32,
        face: i32,
        hair: i32,
        items: Vec<i32>
    ) -> Result<(), Error> {
        let pool = &*get_db();
        let mut tx = pool.begin().await?;

        // TODO handle mercedes creation and shit lol
        let avatar_look = AvatarLookSerializer::create_query(
            &mut tx,
            face,
            hair,
            skin
        ).await?;

        let character_stats = CharacterStatsSerializer::create_query(
            &mut tx,
            name,
            gender,
            job,
            sub_job
        ).await?;

        // Create inventories
        let equipped_inventory = InventorySerializer::create_query(
            &mut tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::None // Yeah lol
        ).await?;

        let equip_inventory = InventorySerializer::create_query(
            &mut tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Equip
        ).await?;

        let consume_inventory = InventorySerializer::create_query(
            &mut tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Consume
        ).await?;


        let install_inventory = InventorySerializer::create_query(
            &mut tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Install
        ).await?;

        let etc_inventory = InventorySerializer::create_query(
            &mut tx,
            DEFAULT_INVENTORY_SIZE,
            InvType::Etc
        ).await?;

        let cash_inventory = InventorySerializer::create_query(
            &mut tx,
            MAX_INVENTORY_SIZE,
            InvType::Cash
        ).await?;

        // Create the character
        let character_serializer = sqlx::query_as!(
            CharacterSerializer,
            "INSERT INTO characters (\
                account_id,
                character_stats_id,
                avatar_look_id,
                equipped_inventory,
                equip_inventory,
                consume_inventory,
                install_inventory,
                etc_inventory,
                cash_inventory\
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
            account_id,
            character_stats.id,
            avatar_look.id,
            equipped_inventory.id,
            equip_inventory.id,
            consume_inventory.id,
            install_inventory.id,
            etc_inventory.id,
            cash_inventory.id
        ).fetch_one(&mut *tx).await?;

        for item in items.iter() {
            let inv_type = GAME_DATA.get_inv_type(item.clone());
            match inv_type {
                InvType::Equip => {
                    // Add to equipped items.
                    let equip_op = GAME_DATA.fetch_equipment(item.clone());
                    if let Some(equip) = equip_op {
                        // Create the equipment (: <3
                        let mut item_ser: ItemSerializer = equip.item.clone().into();
                        let mut eq_ser: EquipmentSerializer = equip.into();
                        item_ser.inventory_id = Some(equipped_inventory.id.clone());
                        // Add the item
                        ItemService::create_item(&mut tx, &mut item_ser).await?;
                        eq_ser.item_id = item_ser.id.clone();
                        ItemService::create_item(&mut tx, &mut eq_ser).await?;
                    }
                }
                InvType::None | InvType::Equipped => {},
                _ => {
                    let item_op = GAME_DATA.fetch_item(item.clone());
                    if let Some(item) = item_op {
                        let mut item_ser = item.into();
                        let inv_id: i64 = {
                            match(inv_type) {
                                InvType::Consume => {consume_inventory.id.clone()}
                                InvType::Install => {install_inventory.id.clone()}
                                InvType::Etc => {etc_inventory.id.clone()}
                                InvType::Cash => {cash_inventory.id.clone()},
                                _ => -1
                            }
                        };
                        item_ser.inventory_id = Some(inv_id);
                        // Add the item
                        ItemService::create_item(&mut tx, &mut item_ser).await?;
                    }

                }
            }
        }

        tx.commit().await
    }

    pub async fn is_duplicated_id(character_name: &str) -> bool {
        let pool = &*get_db();
        let res = sqlx::query!(
            "SELECT name FROM character_stats WHERE name = $1",
            character_name
        ).fetch_one(pool).await;

        match res {
            Ok(chara_rec) => true,
            Err(_) => false
        }
    }
}



