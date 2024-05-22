use byteorder::{ByteOrder, LittleEndian};
use chrono::{NaiveDateTime};

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

impl Into<i64> for FungusTime {
    fn into(self) -> i64 {
       self.get_long()
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