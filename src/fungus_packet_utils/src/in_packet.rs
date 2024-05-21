use crate::in_headers::InHeader;
use crate::packet_errors::PacketError;
use byteorder::{ByteOrder, LittleEndian};
use core::fmt;
use std::fmt::Formatter;

pub struct InPacket {
    pub(crate) opcode: InHeader,
    packet: Vec<u8>,
    cursor: usize,
}

impl fmt::Display for InPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "=> IN [{} ({}|0x{:02X})] :: [ {} ]",
            self.opcode,
            self.get_opcode(),
            self.get_opcode(),
            self.packet[2..]
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl InPacket {
    pub fn new(packet: &[u8]) -> Self {
        let opcode_short = LittleEndian::read_i16(&packet[0..2]);

        InPacket {
            packet: packet.to_vec(),
            opcode: InHeader::from(opcode_short),
            cursor: 2,
        }
    }

    fn get_opcode(&self) -> i16 {
        let opcode_bytes = &self.packet[0..2];
        LittleEndian::read_i16(opcode_bytes)
    }

    pub fn get_header(&self) -> InHeader {
        self.opcode.clone()
    }

    pub fn read_byte(&mut self) -> Result<u8, PacketError> {
        if self.cursor + 1 > self.packet.len() {
            Err(PacketError::OutOfBounds)
        } else {
            let bytes = self.packet[self.cursor..self.cursor + 1].to_vec();
            self.cursor += 1;
            Ok(bytes[0])
        }
    }

    pub fn read_short(&mut self) -> Result<i16, PacketError> {
        if self.cursor + 2 > self.packet.len() {
            Err(PacketError::OutOfBounds)
        } else {
            let bytes = self.packet[self.cursor..self.cursor + 2].to_vec();
            self.cursor += 2;
            Ok(LittleEndian::read_i16(&bytes))
        }
    }

    pub fn read_int(&mut self) -> Result<i32, PacketError> {
        if self.cursor + 4 > self.packet.len() {
            Err(PacketError::OutOfBounds)
        } else {
            let bytes = self.packet[self.cursor..self.cursor + 2].to_vec();
            self.cursor += 4;
            Ok(LittleEndian::read_i32(&bytes))
        }
    }

    pub fn read_long(&mut self) -> Result<i64, PacketError> {
        if self.cursor + 8 >= self.packet.len() {
            Err(PacketError::OutOfBounds)
        } else {
            let bytes = self.packet[self.cursor..self.cursor + 2].to_vec();
            self.cursor += 8;
            Ok(LittleEndian::read_i64(&bytes))
        }
    }

    pub fn read_string(&mut self) -> Result<&str, PacketError> {
        let str_length: usize = self.read_short()? as usize;
        if self.cursor + str_length > self.packet.len() {
            Err(PacketError::OutOfBounds)
        } else {
            let bytes = &self.packet[self.cursor..self.cursor + str_length];
            self.cursor += str_length;
            Ok(std::str::from_utf8(bytes).map_err(PacketError::InvalidUtf8)?)
        }
    }

    pub fn read_exact(&mut self, byte_array: &mut Vec<u8>) -> Result<(), PacketError> {
        let arr_size = byte_array.len();
        if self.cursor + arr_size > self.packet.len() {
            Err(PacketError::OutOfBounds)
        } else {
            let bytes = &self.packet[self.cursor..self.cursor + arr_size].to_vec();
            self.cursor += arr_size;
            *byte_array = bytes.clone();
            Ok(())
        }
    }
}
