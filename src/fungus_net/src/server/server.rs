use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use fungus_database::models::user::User;
use fungus_world::world::World;

pub struct Server {
    pub users: HashSet<i32>,
    pub worlds: Arc<RwLock<[World]>>
}

impl Server {
    pub fn new() -> Server {
        let worlds = Arc::new(RwLock::new([
            World::new(0, String::from("SpookyMS"), 1, 1)
        ]));

        Server {
            users: Default::default(),
            worlds
        }
    }

    pub fn is_user_online(&self, uid: i32) -> bool {
        self.users.contains(&uid)
    }

    pub fn add_user(&mut self, user: &User) {
        self.users.insert(user.id.clone());
    }

    pub fn get_worlds(&self) -> Arc<RwLock<[World]>> {
        self.worlds.clone()
    }

}

pub static SERVER_INSTANCE: Lazy<Arc<RwLock<Server>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Server::new()))
});