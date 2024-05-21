use crate::crypto::packet_cipher::PacketCipher;
use crate::in_packet::InPacket;
use crate::out_packet::OutPacket;
use byteorder::{BigEndian, ReadBytesExt};
use fungus_utils::constants::server_constants::{DEFAULT_RIV, DEFAULT_SIV};
use log::info;

pub struct PacketCoder {
    cipher: PacketCipher,
}

impl Default for PacketCoder {
    fn default() -> Self {
        PacketCoder {
            cipher: PacketCipher::new(DEFAULT_SIV.clone(), DEFAULT_RIV.clone()),
        }
    }
}

impl PacketCoder {
    pub fn new(siv: [u8; 4], riv: [u8; 4]) -> Self {
        PacketCoder {
            cipher: PacketCipher::new(siv, riv),
        }
    }

    pub fn get_riv(&self) -> [u8; 16] {
        self.cipher.recv_iv.clone()
    }

    pub fn get_siv(&self) -> [u8; 16] {
        self.cipher.send_iv.clone()
    }

    pub fn encode(&mut self, packet: &OutPacket) -> Vec<u8> {
        // TODO Need to lock this probably, to avoid
        // problems when encoding/decoding, because the ivs will change.
        if !packet.opcode.clone().is_ignored() {
            info!("{}", packet);
        }
        let mut data = packet.get_bytes().to_vec();
        // Gotta get the client or something, wtf?
        let iv = self.get_siv();
        let header_bytes = self.cipher.get_header(data.len(), &iv);

        self.cipher.encrypt_shanda(&mut data);

        let mut data_clone = data.clone();
        self.cipher.crypt(&mut data_clone, &iv);
        self.cipher.set_new_siv();

        let mut full_packet = Vec::from(header_bytes);
        full_packet.extend_from_slice(data_clone.as_slice());
        full_packet
    }

    pub fn decode(&mut self, data: &[u8]) -> InPacket {
        let iv = self.get_riv();
        // Get packet length
        let crypted_len = data[0..4]
            .iter()
            .as_slice()
            .read_i32::<BigEndian>()
            .expect("NOT WTF");
        let buf_len = self.cipher.get_length(crypted_len);

        let mut decrypt_packet: Vec<u8> = data[4..4 + buf_len].iter().as_slice().to_vec();
        self.cipher.crypt(&mut decrypt_packet, &iv);
        self.cipher.set_new_riv();

        self.cipher.decrypt_shanda(&mut decrypt_packet);

        let in_packet = InPacket::new(&decrypt_packet);
        if !in_packet.opcode.clone().is_ignored() {
            info!("{}", in_packet);
        }
        in_packet
    }
}
