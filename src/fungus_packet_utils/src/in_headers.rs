use crate::in_headers::InHeader::UNKNOWN;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter};

#[repr(i16)]
#[derive(Display, EnumIter, AsRefStr, PartialEq, Eq, Debug, Clone, Copy)]
pub enum InHeader {
    BeginSocket = 0,

    // OnLogin
    CharSelect = 12,
    VersionVerify = 20,
    CheckLoginAuthInfo = 21,
    GuestLogin = 22,

    SelectPreviousWorld = 24,
    SelectWorld = 25,
    WorldStatusRequest = 26,
    WorldListRequest = 31,
    RedisplayWorldList = 32,
    EULA = 7,
    WorldInfoRequest = 11,
    Pong = 46,
    ClientStart = 56,
    ClientError = 59,

    UNKNOWN = -1,
}

impl From<i16> for InHeader {
    fn from(value: i16) -> InHeader {
        InHeader::iter()
            .find(|&x| x as i16 == value)
            .unwrap_or(UNKNOWN)
    }
}

impl InHeader {
    pub fn to_u16(self) -> u16 {
        self as u16
    }
}
