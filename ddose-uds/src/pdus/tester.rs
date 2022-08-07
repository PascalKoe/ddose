use super::{RxPdu, TxPdu};

pub const SID_TESTER_REQ: u8 = 0x3E;
pub const SID_TESTER_RES: u8 = 0x7E;

pub struct TesterRequest {}

impl TesterRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl TxPdu for TesterRequest {
    fn sid() -> u8 {
        SID_TESTER_REQ
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::from([SID_TESTER_REQ, 0x00])
    }
}

pub struct TesterResponse {}

impl RxPdu for TesterResponse {
    fn sid() -> u8 {
        SID_TESTER_RES
    }

    fn len_min() -> usize {
        2
    }

    fn len_max() -> usize {
        2
    }

    fn deserialize(data: &[u8]) -> Self {
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], SID_TESTER_RES);
        assert_eq!(data[1], 0x00);

        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::pdus::{RxPdu, TxPdu};

    use super::{TesterRequest, TesterResponse};

    #[test]
    fn serializes_request() {
        let req = TesterRequest::new();
        assert_eq!(req.serialize(), [0x3E, 0x00]);
    }

    #[test]
    fn deserializes_response() {
        let _res = TesterResponse::deserialize(&[0x7E, 0x00]);
    }
}
