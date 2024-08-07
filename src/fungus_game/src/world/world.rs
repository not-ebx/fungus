use std::sync::{Arc};
use tokio::sync::RwLock;
use fungus_packet_utils::out_headers::OutHeader;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_utils::constants::server_constants::{LOGIN_PORT, WORLD_CHANNELS};
use fungus_utils::enums::server_status::ServerStatus;
use crate::world::channel::Channel;

#[derive(Clone)]
pub struct World {
    pub id: i32,
    pub name: String,
    pub exp_wse: i32,
    pub drop_wse: i32,
    pub event_message: String,
    pub char_creation_blocked: bool,
    // Add references to channels, merchants, parties, guilds, allainces, etc.
    // channels: Arc<RwLock<Vec<Channel>>>
    pub channels: usize, // Amount of channels.
}

impl World {
    pub fn new(world_id: i32, name: String, exp_wse: i32, drop_wse: i32) -> Self {
        World{
            id: world_id,
            name,
            exp_wse,
            drop_wse,
            event_message: String::from(""),
            char_creation_blocked: false,
            channels: WORLD_CHANNELS as usize
        }
    }


    pub fn is_full(&self) -> bool{
        // TODO Implement
        false
    }

    pub fn get_status(&self) -> ServerStatus {
        if self.is_full(){
            return ServerStatus::Full;
        }
        ServerStatus::Normal
    }

}