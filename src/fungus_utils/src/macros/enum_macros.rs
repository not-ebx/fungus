
#[macro_export]
macro_rules! impl_try_error_i16 {
($t:ty) => {
        impl TryFrom<i16> for $t {
            type Error = ConvertError;
            fn try_from(value: i16) -> Result<Self, Self::Error> {
                let item_type = <$t>::iter()
                    .find(|&x| x as i16 == value);
                match item_type {
                    Some(_type) => Ok(_type),
                    None => Err(ConvertError::UnknownValue(value.to_string())),
                }
            }
        }
    };
}


#[macro_export]
macro_rules! impl_into_i16 {
($t:ty, $def_val:expr) => {
        impl From<i16> for $t {
            fn from(value: i16) -> Self {
                let item_type = <$t>::iter()
                    .find(|&x| x as i16 == value);
                match item_type {
                    Some(_type) => _type,
                    None => $def_val
                }
            }
        }
    };
}