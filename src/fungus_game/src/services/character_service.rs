use std::collections::HashMap;
use std::sync::Arc;
use log::info;
use sqlx::Error;
use fungus_database::daos::item_dao::CreateGameItem;
use fungus_database::daos::avatar_look_dao::AvatarLookDAO;
use fungus_database::daos::character_dao::CharacterDAO;
use fungus_database::daos::character_stats_dao::CharacterStatsDAO;
use fungus_database::daos::inventory_dao::InventoryDAO;
use fungus_database::daos::item_dao::ItemDAO;
use fungus_database::database::get_db;
use fungus_database::serializers::character_select_serializer::CharacterSelectSerializer;
use fungus_database::serializers::equipment_serializer::EquipmentSerializer;
use fungus_database::serializers::item_serializer::ItemSerializer;
use fungus_utils::enums::inv_type::InvType;
use crate::entities::character::Character;
use crate::entities::equipment::Equipment;
use crate::entities::inventory::Inventory;
use crate::errors::service_errors::ServiceError;
use crate::game_data::game_info::item_utilities::is_weapon;
use crate::services::game_data_service::GameDataService;

pub struct CharacterService {
    character_stats_dao: CharacterStatsDAO,
    character_dao: CharacterDAO,
    avatar_look_dao: AvatarLookDAO,
    inventory_dao: InventoryDAO,
    item_dao: ItemDAO
}

impl CharacterService {
    pub fn new() -> Self {
        CharacterService{
            character_stats_dao: CharacterStatsDAO,
            character_dao: CharacterDAO,
            avatar_look_dao: AvatarLookDAO,
            inventory_dao: InventoryDAO,
            item_dao: ItemDAO,
        }
    }

    pub async fn get_characters_for_selection(&self, account_id: i32) -> Vec<Character> {
        let pool = &*get_db();
        let ch_ser = CharacterDAO.get_characters_for_login_screen(pool, account_id).await;

        // Get the visible items for all of them.
        let mut visible_items: HashMap<i32, Vec<i32>> = HashMap::new();
        for ch in &ch_ser {
            let ch_id = ch.character.id;
            let items = self.get_character_visible_equipment(ch_id).await;
            visible_items.insert(ch_id, items);
        }

        let characters: Vec<Character> = ch_ser.into_iter()
            .map(|ch| {
                let mut chara = Character::from(ch);
                chara.avatar_look.visible_equipment = visible_items.get(&chara.id).unwrap_or(&Vec::default()).to_owned();
                chara
            })
            .collect();

        // For each character, we need to get the equipped items.
        characters
    }

    pub async fn get_character_visible_equipment(&self, character_id: i32) -> Vec<i32> {
        let pool = &*get_db();
        self.inventory_dao.find_characters_equipped_items(pool, character_id).await
    }

    pub async fn create_character(
        &self,
        game_data_service: Arc<GameDataService>,
        account_id: i32,
        name: &str,
        job: i32,
        sub_job: i16,
        gender: u8,
        skin: i32,
        face: i32,
        hair: i32,
        items: Vec<i32>
    ) -> Result<Character, ServiceError> {
        let pool = &*get_db();
        let mut tx = pool.begin().await?;

        // TODO handle mercedes creation and shit lol
        let mut avatar_look = self.avatar_look_dao.create_query(
            &mut tx,
            face,
            hair,
            skin,
            gender as i16,
            job,
        ).await?;

        let character_stats = self.character_stats_dao.create_query(
            &mut tx,
            name,
            gender,
            job,
            sub_job
        ).await?;

        // Equipped, equip, use, install, etc, cash = 6
        let inventories = self.inventory_dao.create_inventory(
            &mut tx,
        ).await?;

        let mut visible_equipment: Vec<i32> = vec![];

        for item in items.iter() {
            let inv_type = game_data_service.get_inv_type(item.clone());
            match inv_type {
                InvType::Equip => {
                    // Add to equipped items.
                    let equip_op = game_data_service.fetch_equipment(item.clone());
                    if let Some(equip) = equip_op {
                        // Create the equipment (: <3
                        let mut item_ser: ItemSerializer = equip.item.clone().into();
                        let mut eq_ser: EquipmentSerializer = equip.into();
                        item_ser.inventory_id = Some(inventories[0].clone());
                        // Add the item
                        self.item_dao.create_item(&mut tx, &mut item_ser).await?;
                        eq_ser.item_id = item_ser.id.clone();
                        self.item_dao.create_item(&mut tx, &mut eq_ser).await?;

                        if is_weapon(item_ser.item_id) {
                            avatar_look.weapon_sticker_id = Some(item_ser.item_id);
                            avatar_look.weapon_id = Some(item_ser.item_id);
                        }
                        visible_equipment.push(item_ser.item_id);

                    }
                }
                InvType::None | InvType::Equipped => {},
                _ => {
                    let item_op = game_data_service.fetch_item(item.clone());
                    if let Some(item) = item_op {
                        let mut item_ser: ItemSerializer = item.into();
                        let inv_id: i64 = {
                            match(inv_type) {
                                InvType::Consume => {inventories[2].clone()}
                                InvType::Install => {inventories[3].clone()}
                                InvType::Etc => {inventories[4].clone()}
                                InvType::Cash => {inventories[5].clone()},
                                _ => -1
                            }
                        };
                        item_ser.inventory_id = Some(inv_id);
                        // Add the item
                        self.item_dao.create_item(&mut tx, &mut item_ser).await?;
                    }

                }
            }
        }

        // Finally, create the character

        //         // Equipped, equip, use, install, etc, cash = 6
        let new_character = self.character_dao.create(
            &mut tx,
            account_id,
            character_stats.id,
            avatar_look.id,
            inventories[0],
            inventories[1],
            inventories[2],
            inventories[3],
            inventories[4],
            inventories[5],
        ).await?;

        info!("Created character {} with id {}", character_stats.name.clone(), new_character.id);

        let _ = tx.commit().await;

        // since it's created, we now create the character and return it
        let csch_serialized = CharacterSelectSerializer {
            character: new_character,
            avatar_look,
            character_stats
        };

        let mut chara = Character::from(csch_serialized);
        chara.avatar_look.visible_equipment = visible_equipment;

        Ok(chara)
    }

    pub async fn is_duplicated_id(&self, character_name: &str) -> bool {
        let pool = &*get_db();

        self.character_stats_dao.is_name_taken(
            pool,
            character_name
        ).await
    }

    pub async fn get_character(&self, character_id: i32) -> Result<Character, ServiceError> {
        let pool = &*get_db();
        let chara = self.character_dao.get_character_by_id(pool, character_id).await?;

        Ok(Character::from(chara))
    }


    pub async fn get_account_characters(&self, account_id: i32) -> Vec<Character> {
        vec![]
    }
}