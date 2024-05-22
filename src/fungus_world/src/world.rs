use std::sync::{Arc};
use tokio::sync::RwLock;
use fungus_packet_utils::out_headers::OutHeader;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_utils::constants::server_constants::{LOGIN_PORT, WORLD_CHANNELS};
use fungus_utils::enums::server_status::ServerStatus;
use crate::channel::Channel;

#[derive(Clone)]
pub struct World {
    pub id: i32,
    name: String,
    exp_wse: i32,
    drop_wse: i32,
    event_message: String,
    char_creation_blocked: bool,
    // Add references to channels, merchants, parties, guilds, allainces, etc.
    channels: Arc<RwLock<Vec<Channel>>>
}

impl World {
    pub fn new(world_id: i32, name: String, exp_wse: i32, drop_wse: i32) -> Self {
        // Create the channels first
        let mut channels = Vec::with_capacity(WORLD_CHANNELS as usize);
        for i in 0..WORLD_CHANNELS {
            let ch = Channel::new(
                i,
                world_id,
                String::from(format!("{}-{}", name.clone(), i)),
                LOGIN_PORT + (100 * world_id) + i
            );
            channels.push(ch);
        }

        World{
            id: world_id,
            name,
            exp_wse,
            drop_wse,
            event_message: String::from(""),
            char_creation_blocked: false,
            channels: Arc::new(RwLock::new(channels))
        }
    }

    pub fn get_channels(&self) -> Arc<RwLock<Vec<Channel>>> {
        self.channels.clone()
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

    pub async fn get_info_as_packet(&self) -> Vec<u8> {
        let mut packet = OutPacket::new(OutHeader::WorldInformation);
        packet.write_byte(self.id as u8);
        packet.write_string(self.name.clone());
        packet.write_byte(0); // World state?
        packet.write_string(self.event_message.clone()); // World Event Description
        packet.write_short(self.exp_wse as i16);
        packet.write_short(self.drop_wse as i16);
        packet.write_bool(self.char_creation_blocked);

        // Encode Channels
        packet.write_byte(self.channels.read().await.len() as u8);
        for channel in self.channels.read().await.iter() {
            packet.write_string(channel.name.clone());
            packet.write_int(channel.get_gauge_px());
            packet.write_byte(channel.world_id as u8);
            packet.write_byte(channel.id as u8);
            packet.write_bool(channel.is_adult_channel);
        }

        // Something about "string infos"
        // Lets jsut encode 0 lol
        packet.write_short(0);


        packet.write_int(0); // offset
        packet.as_bytes()
    }
}