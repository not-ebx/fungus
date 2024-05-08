use crate::in_headers::InHeader;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter};

#[repr(i16)]
#[derive(Display, EnumIter, AsRefStr, PartialEq, Eq, Debug, Clone, Copy)]
pub enum OutHeader {
    CheckPasswordResult = 0,
    GuestIDLoginResult = 1,
    AccountInfoResult = 2,
    CheckUserLimitResult = 3,
    SetAccountResult = 4,
    ConfirmEULAResult = 5,
    // ...
    WorldInformation = 10,
    SelectWorldResult = 11,
    //
    //ClientStart = 14,
    // .. More
    AliveReq = 17,
    // .. more xd
    UNKNOWN = -1,
}

impl From<i16> for OutHeader {
    fn from(value: i16) -> OutHeader {
        OutHeader::iter()
            .find(|&x| x as i16 == value)
            .unwrap_or(OutHeader::UNKNOWN)
    }
}

impl OutHeader {
    pub fn to_u16(self) -> u16 {
        self as u16
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }
}
