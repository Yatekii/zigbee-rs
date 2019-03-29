pub trait Serde<T, E> {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, E>;
    fn deserialize(data: &[u8]) -> Result<T, E>;
}