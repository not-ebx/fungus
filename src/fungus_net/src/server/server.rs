use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use once_cell::sync::Lazy;
use fungus_game::entities::character::Character;
use fungus_game::entities::user::User;
use fungus_game::services::service_registry::ServiceRegistry;
use fungus_utils::fg_printc_info;
use fungus_game::world::world::World;
use crate::session::client_session::ClientSession;

pub struct Server {
    pub users: HashSet<i32>,
    sessions: HashMap<String, Arc<Mutex<ClientSession>>>,

    // Stuff
    pub service_registry: Arc<ServiceRegistry>,
}

impl Server {
    pub fn new(service_registry: Arc<ServiceRegistry>) -> Server {
        Server {
            users: Default::default(),
            sessions: Default::default(),
            service_registry
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

    pub fn get_starting_items(&self) -> HashSet<i32> {
        self.service_registry.get_game_data_service().etc_data.starting_items.clone()
    }

}