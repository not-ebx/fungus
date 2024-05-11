use byteorder::{ByteOrder, LittleEndian};
use chrono::{NaiveDateTime};
use crate::traits::encodable::{Encodable};

pub struct FungusTime {
    time: NaiveDateTime
}

impl From<NaiveDateTime> for FungusTime {
    fn from(value: NaiveDateTime) -> Self {
        FungusTime{
            time: value
        }
    }
}

impl Encodable for FungusTime {
    fn encode(&self) -> Vec<u8> {
        let as_long = self.get_long();
        let mut as_bytes: [u8; 8] = [0;8];
        LittleEndian::write_i64(&mut as_bytes, as_long);

        as_bytes.to_vec()
    }
}

impl FungusTime {
    pub fn get_long(&self) -> i64 {
        self.time.and_utc().timestamp()
    }

    pub fn get_high_part(&self) -> [u8; 4] {
        let as_bytes = self.encode();
        let mut high_part: [u8; 4] = [0;4];
        high_part.copy_from_slice(&as_bytes[0..4]);

        high_part
    }

    pub fn get_low_part(&self) -> [u8; 4] {
        let as_bytes = self.encode();
        let mut low_part: [u8; 4] = [0; 4];
        low_part.copy_from_slice(&as_bytes[4..]);

        low_part
    }
}