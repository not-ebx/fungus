use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter};
use crate::errors::convert_error::ConvertError;
use crate::{impl_try_error_i16};


#[repr(i16)]
#[derive(Display, EnumIter, AsRefStr, PartialEq, Eq, Debug, Clone, Copy)]
pub enum InvType {
    Equipped = -1,
    None = 0, // Floor, dropped, whatever.
    Equip = 1,
    Consume = 2,
    Install = 3,
    Etc = 4,
    Cash = 5,
}

impl Into<u8> for InvType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<&str> for InvType {
    fn from(value: &str) -> Self {
        match value {
            "cash" | "pet" => {
                InvType::Cash
            }
            "consume" | "special" | "use" => {
                InvType::Consume
            }
            "etc" => {
                InvType::Etc
            }
            "install" | "setup" => {
                InvType::Install
            }
            "eqp" | "equip" => {
                InvType::Equip
            }
            _ => {
                InvType::None
            }
        }
    }
}

impl_try_error_i16!(InvType);
