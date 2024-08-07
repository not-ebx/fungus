use std::collections::HashMap;
use crate::world::channel::Channel;

pub struct ChannelService {
    channels: HashMap<i32, Vec<Channel>>
}

impl ChannelService {
    pub fn new(channels: HashMap<i32, Vec<Channel>>) -> Self {
        ChannelService {
            channels
        }
    }

    pub fn set_channels(&mut self, world_id: i32, channels: Vec<Channel>) {
        self.channels.insert(world_id, channels);
    }

    pub fn get_world_channels(&self, world_id: i32) -> Vec<Channel> {
        self.channels.get(&world_id).unwrap_or(&Vec::new()).to_vec()
    }
}