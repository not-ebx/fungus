use std::collections::HashSet;
use crate::in_headers::InHeader::UNKNOWN;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter};
use once_cell::sync::Lazy;

#[repr(i16)]
#[derive(Display, EnumIter, AsRefStr, PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum InHeader {
    BeginSocket = 0,
    CheckLoginAuthInfo = 1,
    GuestLogin = 2,
    AccountInfoRequest = 3,
    WorldInfoRequest = 4,
    SelectWorld = 5,
    WorldStatusRequest = 6, // Check user limit
    EULA = 7,
    SetGender = 8,
    CheckPinCode = 9,
    UpdatePinCode = 10,
    WorldListRequest = 11, // WorldRequest
    RedisplayWorldList = 12, // Logout world
    ViewAllChar = 13,
    SelectCharacterByVAC = 14,
    VACFlagSet = 15,

    CheckTransferWorldPossible = 18, // ?? Unused, most likely.
    CharSelect = 19,
    MigrateIn = 20,
    CheckDuplicateID = 21,
    CreateNewCharacter = 22,
    CreateNewCharacterInCS = 23,

    // OnLogin
    VersionVerify = 34, // CreateSecurityHandle
    SelectPreviousWorld = 24,
    CharSelectNoPic = 38, // CreateNewCharaterEx

    Pong = 25, // Alive Ack?

    ClientStart = 27, // Security Packet
    ClientError = 36, // Exception Log

    ///
    ///
    ///
    UpdateScreenSetting = 218,

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
