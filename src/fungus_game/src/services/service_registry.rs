use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use log::info;
use fungus_database::database::get_db;
use fungus_utils::{fg_printc_error, fg_printc_info};
use fungus_utils::constants::server_constants::{LOGIN_PORT, WORLD_CHANNELS};
use crate::services::account_service::AccountService;
use crate::services::channel_service::ChannelService;
use crate::services::character_service::CharacterService;
use crate::services::game_data_service::GameDataService;
use crate::services::item_service::ItemService;
use crate::services::user_service::UserService;
use crate::services::world_service::WorldService;
use crate::world::channel::Channel;
use crate::world::world::World;

pub struct ServiceRegistry {
    account_service: Arc<AccountService>,
    user_service: Arc<UserService>,
    item_service: Arc<ItemService>,
    game_data_service: Arc<GameDataService>,
    character_service: Arc<CharacterService>,
    channel_service: Arc<ChannelService>,
    world_service: Arc<WorldService>
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

        fg_printc_info!("Setting up WorldService");
        let world_service = {
            let world = World::new(0, String::from("SpookyMS"), 1, 1);
            let mut service = WorldService::new();
            service.add_world(world);

            service
        };

        fg_printc_info!("Setting up ChannelService");
        let channel_service = {
            let worlds = world_service.get_worlds();
            let mut world_channels: HashMap<i32, Vec<Channel>> = HashMap::new();
            for world in worlds {
                let mut channels = Vec::with_capacity(WORLD_CHANNELS as usize);
                for i in 0..WORLD_CHANNELS {
                    let ch = Channel::new(
                        i,
                        world.id,
                        String::from(format!("{}-{}", world.name.clone(), i)),
                        LOGIN_PORT + (100 * world.id) + i
                    );
                    channels.push(ch);
                }
                world_channels.insert(world.id.clone(), channels);
            }
            ChannelService::new(world_channels)
        };

        let sr = ServiceRegistry {
            account_service: Arc::new(AccountService::new()),
            user_service: Arc::new(UserService::new()),
            item_service: Arc::new(ItemService::new()),
            game_data_service: Arc::new(game_data_service),
            character_service: Arc::new(CharacterService::new()),
            channel_service: Arc::from(channel_service),
            world_service: Arc::from(world_service)
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

    pub fn get_world_service(&self) -> Arc<WorldService> {
        self.world_service.clone()
    }

    pub fn get_channel_service(&self) -> Arc<ChannelService> {
        self.channel_service.clone()
    }
}