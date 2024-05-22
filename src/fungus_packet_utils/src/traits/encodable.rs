use crate::out_packet::OutPacket;

pub trait Encodable {
    fn encode(&self, out_packet: &mut OutPacket);
}