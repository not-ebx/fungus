use crate::out_headers::OutHeader;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use core::fmt;
use fungus_utils::traits::encodable::Encodable;
use log::{error, info};
use std::fmt::Formatter;
use std::io;
use std::io::Write;
use std::time::SystemTime;

pub struct OutPacket {
    pub opcode: OutHeader,
    pub packet: Vec<u8>,
}

impl From<Vec<u8>> for OutPacket {
    fn from(value: Vec<u8>) -> Self {
        let out_header = &value[0..2];
        let packet_header = OutHeader::from(
            LittleEndian::read_i16(out_header)
        );
        if packet_header.eq(&OutHeader::UNKNOWN) {
            error!("OutPacket::from<Vec<u8>> MUST come from a KNOWN OutHeader. Returning empty outpacket.");
            return OutPacket::default();
        }

        let mut out_packet = OutPacket::new(packet_header);
        out_packet.write_bytes(&value[2..]);

        out_packet

    }
}

impl Default for OutPacket {
    fn default() -> Self {
        OutPacket {
            opcode: OutHeader::UNKNOWN,
            packet: vec![],
        }
    }
}

impl fmt::Display for OutPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let packet_slice = self.packet.get(2..).unwrap_or(&[]);
        let packet_stringified = packet_slice
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");

        write!(
            f,
            "<= OUT [{} ({}|0x{:02X})] :: [ {} ]",
            self.opcode,
            self.get_opcode(),
            self.get_opcode(),
            packet_stringified
        )
    }
}

impl OutPacket {
    pub fn new(header: OutHeader) -> Self {
        let opcode = header.clone();
        let mut packet_arr: Vec<u8> = Vec::new();
        let header_bytes: [u8; 2] = header.to_u16().to_le_bytes();
        packet_arr.extend_from_slice(&header_bytes);

        OutPacket {
            packet: packet_arr,
            opcode,
        }
    }

    pub fn from_encodable<T: Encodable>(value: T) -> Self {
        let vec_packet = value.encode();
        OutPacket::from(vec_packet)
    }

    fn get_opcode(&self) -> i16 {
        let opcode_bytes = self.packet.get(0..2).unwrap_or(&[0, 0]);
        LittleEndian::read_i16(opcode_bytes)
    }

    pub fn write<T: Encodable>(&mut self, value: T) {
        self.write_bytes(value.encode().as_slice());
    }

    pub fn write_bool(&mut self, value: bool) {
        self.packet.push(value as u8);
    }

    pub fn write_byte(&mut self, value: u8) {
        self.packet.push(value);
    }

    pub fn write_short(&mut self, value: i16) {
        let value_bytes: [u8; 2] = value.to_le_bytes();
        self.packet.extend_from_slice(&value_bytes);
    }

    pub fn write_int(&mut self, value: i32) {
        let value_bytes: [u8; 4] = value.to_le_bytes();
        self.packet.extend_from_slice(&value_bytes);
    }

    pub fn write_long(&mut self, value: i64) {
        let value_bytes: [u8; 8] = value.to_le_bytes();
        self.packet.extend_from_slice(&value_bytes);
    }

    pub fn write_string(&mut self, value: String) {
        let length_bytes = value.len() as u16;
        // TODO check that the length is more than max short size.
        let str_bytes: &[u8] = value.as_bytes();

        self.write_short(length_bytes as i16);
        let res = self.packet.write_all(&str_bytes);
    }

    pub fn write_bytes(&mut self, value: &[u8]) {
        self.packet.extend_from_slice(value);
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.packet
    }

    // this will change ownership.
    pub fn as_bytes(self) -> Vec<u8> {
        self.packet
    }
}
