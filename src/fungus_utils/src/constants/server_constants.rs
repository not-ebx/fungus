use std::string::ToString;

pub const ARGON_SALT: &[u8] = "strong salt and pepper lmao".as_bytes();
pub const SERVER_IP: &str = "192.168.1.85";
pub const LOGIN_PORT: i32 = 8484;
pub const VERSION: i16 = 111;
pub const MINOR_VERSION: &str = "1";
pub const LOCALE: u8 = 8;
pub const DEFAULT_SIV: [u8;4] = [70,114,122,82];
pub const DEFAULT_RIV: [u8;4] = [82,48,120,115];
pub const MAX_PACKET_SIZE: usize = 65535;
pub const ALLOW_AUTO_REGISTER: bool = true;