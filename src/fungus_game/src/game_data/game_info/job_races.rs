use strum_macros::{EnumIter, FromRepr};

#[derive(Debug, Eq, PartialEq, EnumIter, FromRepr)]
#[repr(i32)]
pub enum JobRace {
    Resistance = 0,
    Explorer = 1,
    Cygnus = 2,
    Aran = 3,
    Evan = 4,
    Mercedes = 5,
    Demon = 6,
    Phantom = 7,
    DualBlade = 8
}

impl From<i32> for JobRace {
    fn from(value: i32) -> Self {
        JobRace::from_repr(value).unwrap_or(JobRace::Explorer)
    }
}