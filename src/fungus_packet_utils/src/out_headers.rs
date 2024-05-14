use std::collections::HashSet;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter};

#[repr(i16)]
#[derive(Display, EnumIter, AsRefStr, PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum OutHeader {
    CheckPasswordResult = 0,
    GuestIDLoginResult = 1,
    AccountInfoResult = 2,
    CheckUserLimitResult = 3,
    SetAccountResult = 4,
    ConfirmEULAResult = 5,
    CheckPinResult = 6,
    UpdatePinResult = 7,
    VacResult = 8,
    SelectByVACResult = 9,
    WorldInformation = 10,
    SelectWorldResult = 11,
    SelectCharacterResult = 12,
    CheckDuplicatedIdResult = 13,
    CreateNewCharacterResult = 14,
    DeleteCharacterResult = 15,
    MigrateCommand = 16,
    AliveReq = 17,
    // .. more xd
    LatestConnectedWorld = 27,
    RecommendedWorldMessage = 28,
    UNKNOWN = -1,
}

static IGNORED_HEADERS: Lazy<HashSet<OutHeader>> = Lazy::new(||
    HashSet::from([
        OutHeader::AliveReq
    ])
);

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

    pub fn is_ignored(&self) -> bool {
        IGNORED_HEADERS.contains(self)
    }
}
