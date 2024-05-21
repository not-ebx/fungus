#[repr(u8)]
pub enum CharacterIDResult {
    Available = 0,
    InUse = 1,
    Invalid = 2,
    InvalidCash = 3 // ??? what
}