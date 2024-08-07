use crate::world::world::World;

pub struct WorldService {
    worlds: Vec<World>
}

impl WorldService {
    pub fn new() -> Self {
        WorldService {
            worlds: vec![]
        }
    }

    pub fn get_world_ids(&self) -> Vec<i32> {
        self.worlds.as_slice().iter().map(|w| w.id.clone()).collect()
    }

    pub fn get_worlds(&self) -> Vec<World> {
        self.worlds.as_slice().iter().map(|w| w.clone()).collect()
    }

    pub fn add_world(&mut self, world: World) {
        self.worlds.push(world);
    }

    pub fn get_world(&self, world_id: i32) -> Option<World> {
        let world_option = self.worlds.iter().find(|&w| w.id.clone() == world_id);
        if let Some(world) = world_option {
            Some(world.clone().to_owned())
        } else {
            None
        }
    }
}