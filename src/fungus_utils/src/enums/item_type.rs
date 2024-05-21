use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter};
use crate::errors::convert_error::ConvertError;
use crate::{impl_try_error_i16};

#[repr(i16)]
#[derive(Display, EnumIter, AsRefStr, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ItemType {
    Equip = 1,
    Item = 2, //Could be use, install, etc
    Pet = 3
}

impl Into<u8> for ItemType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl_try_error_i16!(ItemType);