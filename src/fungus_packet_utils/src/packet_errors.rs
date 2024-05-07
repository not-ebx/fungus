use std::fmt;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum PacketError {
    OutOfBounds,
    UnhandledHeader(String),
    UnknownHeader(String),
    UnimplementedPacket(String),
    InvalidUtf8(Utf8Error),
    InvalidCipher(),
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PacketError::OutOfBounds => {
                write!(f, "Error: Can't move cursor out of the bounds of the packet")
            }
            PacketError::UnhandledHeader(value) => {
                write!(f, "Error: Unhandled Header {}", value)
            }
            PacketError::UnknownHeader(value) => {
                write!(f, "Error: Unknown Opcode ({})", value)
            }
            PacketError::InvalidUtf8(value) => {
                write!(f, "Error: Invalid utf8 string ({})", value)
            }
            PacketError::UnimplementedPacket(value) => {
                write!(f, "Error: The packet is not implemented ({})", value)
            }
            _ => {
                write!(f, "Unhandled Error happened :(")
            }
        }
    }
}