pub trait Encodable {
    fn encode(&self) -> Vec<u8>;
}