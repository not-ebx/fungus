use std::fmt;
use strum_macros::{AsRefStr, Display, EnumIter};
use crate::in_headers::InHeader::{BeginSocket, CheckLoginAuthInfo, EULA, GuestLogin, SelectPreviousWorld, SelectWorld, UNKNOWN, WorldInfoRequest};

#[repr(i16)]
#[derive(Display, EnumIter, AsRefStr, Debug, Clone)]
pub enum InHeader {
    BeginSocket = 0,
    CheckLoginAuthInfo = 1,
    GuestLogin = 2,

    SelectPreviousWorld = 4,
    SelectWorld = 5,
    WorldStatusRequest = 6,
    EULA = 7,



    WorldInfoRequest = 11,
    UNKNOWN = -1
}

impl From<i16> for InHeader {
    fn from(value: i16) -> Self {
        match value {
            0 => BeginSocket,
            1 => CheckLoginAuthInfo,
            2 => GuestLogin,
            4 => SelectPreviousWorld,
            5 => SelectWorld,
            7 => EULA,
            11 => WorldInfoRequest,
            _ => UNKNOWN
        }
    }
}

impl InHeader {
    pub fn to_u16(self) -> u16 {
        self as u16
    }
}
