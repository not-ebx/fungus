use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use fungus_game::entities::user::User;
use fungus_game::services::service_registry::ServiceRegistry;
use fungus_utils::fg_printc_info;
use fungus_world::world::World;

pub struct Server {
    pub users: HashSet<i32>,
    pub worlds: Arc<RwLock<Vec<World>>>,

    // Stuff
    pub service_registry: Arc<ServiceRegistry>,
}

impl Server {
    pub fn new() -> Server {
        fg_printc_info!("Starting Fungus v0.0.1 Alpha. What the sigma");
        let worlds = Arc::new(RwLock::new(vec![
            World::new(0, String::from("SpookyMS"), 1, 1)
        ]));

        Server {
            users: Default::default(),
            worlds,
            service_registry: Arc::from(ServiceRegistry::new())
        }
    }

    pub fn get_services_registry(&self) -> Arc<ServiceRegistry> {
        self.service_registry.clone()
    }

    pub fn is_user_online(&self, uid: i32) -> bool {
        self.users.contains(&uid)
    }

    pub fn add_user(&mut self, user: &User) {
        self.users.insert(user.id.clone());
    }

    pub fn get_worlds(&self) -> Arc<RwLock<Vec<World>>> {
        self.worlds.clone()
    }

    pub fn get_starting_items(&self) -> HashSet<i32> {
        self.service_registry.get_game_data_service().etc_data.starting_items.clone()
    }
}