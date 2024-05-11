use std::cmp::max;
use std::collections::HashMap;
use fungus_utils::constants::server_constants::MAX_CHANNEL_CONNECTIONS;
use crate::world::World;

pub struct Channel {
    pub id: i32,
    pub world_id: i32,
    pub port: i32,
    pub name: String, // Usually WorldName-ChannelId
    max_connections: i32,
    pub is_adult_channel: bool,

    // TODO Create characters
    characters: HashMap<i32, i32>,
}

impl Channel {
    pub fn new(id: i32, world_id: i32, channel_name: String, channel_port: i32) -> Self {
        Channel {
            id,
            world_id,
            port: channel_port,
            name: channel_name,
            max_connections: MAX_CHANNEL_CONNECTIONS,
            is_adult_channel: false,
            characters: Default::default(),
        }
    }

    pub fn get_gauge_px(&self) -> i32{
        max(1, (self.characters.len() as i32 / self.max_connections))
    }
}