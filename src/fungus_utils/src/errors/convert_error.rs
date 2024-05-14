use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum ConvertError {
    UnknownType,
    UnknownValue(String)
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::UnknownType=> {
                write!(
                    f,
                    "Error: Can't convert from input type"
                )
            }
            ConvertError::UnknownValue(value) => {
                write!(
                    f,
                    "Error: Value \"{}\" is unknown for enum",
                    value
                )
            }
        }
    }
}