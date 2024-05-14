use std::collections::HashSet;
use crate::in_headers::InHeader::UNKNOWN;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter};
use once_cell::sync::Lazy;

#[repr(i16)]
#[derive(Display, EnumIter, AsRefStr, PartialEq, Eq, Debug, Clone, Copy, Hash)]
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

static IGNORED_HEADERS: Lazy<HashSet<InHeader>> = Lazy::new(||
    HashSet::from([
        InHeader::Pong
    ])
);

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

    pub fn is_ignored(&self) -> bool {
        IGNORED_HEADERS.contains(self)
    }
}
