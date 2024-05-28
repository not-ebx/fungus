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
use crate::errors::service_errors::ServiceError;
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
        let characters: Vec<Character> = ch_ser.into_iter()
            .map(|ch| Character::from(ch))
            .collect();

        characters
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
        let avatar_look = self.avatar_look_dao.create_query(
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

        tx.commit().await;

        // since it's created, we now create the character and return it
        let csch_serialized = CharacterSelectSerializer {
            character: new_character,
            avatar_look,
            character_stats
        };

        Ok(Character::from(csch_serialized))
    }

    pub async fn is_duplicated_id(&self, character_name: &str) -> bool {
        let pool = &*get_db();

        self.character_stats_dao.is_name_taken(
            pool,
            character_name
        ).await
    }

    pub async fn get_account_characters(&self, account_id: i32) -> Vec<Character> {
        vec![]
    }
}