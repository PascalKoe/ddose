pub mod download;
pub mod dsc;
pub mod ecu_reset;
pub mod routine;
pub mod security_access;
pub mod tester;
pub mod transfer;

pub trait TxPdu {
    fn sid() -> u8;
    fn serialize(&self) -> Vec<u8>;
}

pub trait RxPdu {
    fn sid() -> u8;
    fn len_min() -> usize;
    fn len_max() -> usize;
    fn deserialize(data: &[u8]) -> Self;
}
