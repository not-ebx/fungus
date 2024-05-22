use std::sync::Arc;
use std::time::Instant;
use log::info;
use fungus_database::database::get_db;
use fungus_utils::{fg_printc_error, fg_printc_info};
use crate::services::account_service::AccountService;
use crate::services::character_service::CharacterService;
use crate::services::game_data_service::GameDataService;
use crate::services::item_service::ItemService;
use crate::services::user_service::UserService;

pub struct ServiceRegistry {
    account_service: Arc<AccountService>,
    user_service: Arc<UserService>,
    item_service: Arc<ItemService>,
    game_data_service: Arc<GameDataService>,
    character_service: Arc<CharacterService>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        fg_printc_info!("Setting up the Service Registry");
        let total_time = Instant::now();
        fg_printc_info!("Starting Game Data Service");
        let game_data_service = GameDataService::new();

        fg_printc_info!("Starting Game-Related database fetch services");
        // Test database
        let _db_test = &*get_db();
        if _db_test.is_closed() {
            fg_printc_error!("Database connection is not open.");
            panic!();
        }

        let sr = ServiceRegistry {
            account_service: Arc::new(AccountService::new()),
            user_service: Arc::new(UserService::new()),
            item_service: Arc::new(ItemService::new()),
            game_data_service: Arc::new(game_data_service),
            character_service: Arc::new(CharacterService::new())
        };

        let startup_duration = Instant::now() - total_time;
        fg_printc_info!("Setting up the Service Registry took {}ms", startup_duration.as_millis());
        sr
    }


    pub fn get_game_data_service(&self) -> Arc<GameDataService> {
        self.game_data_service.clone()
    }
    pub fn get_account_service(&self) -> Arc<AccountService> {
        self.account_service.clone()
    }

    pub fn get_user_service(&self) -> Arc<UserService> {
        self.user_service.clone()
    }

    pub fn get_item_service(&self) -> Arc<ItemService> {
        self.item_service.clone()
    }

    pub fn get_character_service(&self) -> Arc<CharacterService> {
        self.character_service.clone()
    }
}