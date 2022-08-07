use super::{RxPdu, TxPdu};

pub const SID_TRANSFER_REQ: u8 = 0x36;
pub const SID_TRANSFER_RES: u8 = 0x76;
pub const SID_TRANSFER_EXIT_REQ: u8 = 0x37;
pub const SID_TRANSFER_EXIT_RES: u8 = 0x77;

pub struct TransferRequest<'a> {
    block_seq_counter: u8,
    payload: &'a [u8],
}

impl<'a> TransferRequest<'a> {
    pub fn new(block_seq_counter: impl Into<u8>, data: &'a [u8]) -> Self {
        Self {
            block_seq_counter: block_seq_counter.into(),
            payload: data,
        }
    }
}

impl<'a> TxPdu for TransferRequest<'a> {
    fn sid() -> u8 {
        SID_TRANSFER_REQ
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::from([SID_TRANSFER_REQ, self.block_seq_counter]);
        buffer.extend_from_slice(self.payload);
        buffer
    }
}

pub struct TransferResponse {
    pub block_seq_counter: u8,
    pub payload: Vec<u8>,
}

impl RxPdu for TransferResponse {
    fn sid() -> u8 {
        SID_TRANSFER_RES
    }

    fn len_min() -> usize {
        2
    }

    fn len_max() -> usize {
        usize::MAX
    }

    fn deserialize(data: &[u8]) -> Self {
        assert!(data.len() >= 2);
        assert_eq!(data[0], SID_TRANSFER_RES);

        let payload = Vec::from(&data[2..]);
        Self {
            block_seq_counter: data[1],
            payload,
        }
    }
}

pub struct TransferExitRequest<'a> {
    payload: &'a [u8],
}

impl<'a> TransferExitRequest<'a> {
    pub fn new(payload: &'a [u8]) -> Self {
        Self { payload }
    }
}

impl<'a> TxPdu for TransferExitRequest<'a> {
    fn sid() -> u8 {
        SID_TRANSFER_EXIT_REQ
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::from([SID_TRANSFER_EXIT_REQ]);
        buffer.extend_from_slice(self.payload);
        buffer
    }
}

pub struct TransferExitResponse {
    pub payload: Vec<u8>,
}

impl RxPdu for TransferExitResponse {
    fn sid() -> u8 {
        SID_TRANSFER_EXIT_RES
    }

    fn len_min() -> usize {
        1
    }

    fn len_max() -> usize {
        usize::MAX
    }

    fn deserialize(data: &[u8]) -> Self {
        assert!(!data.is_empty());
        assert_eq!(data[0], SID_TRANSFER_EXIT_RES);

        Self {
            payload: Vec::from(&data[1..]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pdus::{
        transfer::{TransferExitRequest, TransferExitResponse},
        RxPdu, TxPdu,
    };

    use super::{TransferRequest, TransferResponse};

    #[test]
    fn serializes_request() {
        let req = TransferRequest::new(0x01, &[]);
        assert_eq!(req.serialize(), [0x36, 0x01]);
        let req = TransferRequest::new(0x02, &[0x00]);
        assert_eq!(req.serialize(), [0x36, 0x02, 0x00]);
        let req = TransferRequest::new(0x03, &[0xFF, 0xFF]);
        assert_eq!(req.serialize(), [0x36, 0x03, 0xFF, 0xFF]);
    }

    #[test]
    fn deserializes_response() {
        let res = TransferResponse::deserialize(&[0x76, 0x01]);
        assert_eq!(res.block_seq_counter, 0x01);
        assert_eq!(res.payload, []);
        let res = TransferResponse::deserialize(&[0x76, 0x02, 0xFF]);
        assert_eq!(res.block_seq_counter, 0x02);
        assert_eq!(res.payload, [0xFF]);
        let res = TransferResponse::deserialize(&[0x76, 0x03, 0xFF, 0xFF]);
        assert_eq!(res.block_seq_counter, 0x03);
        assert_eq!(res.payload, [0xFF, 0xFF]);
    }

    #[test]
    fn serialize_exit_request() {
        let req = TransferExitRequest::new(&[]);
        assert_eq!(req.serialize(), [0x37]);
        let req = TransferExitRequest::new(&[0xFF]);
        assert_eq!(req.serialize(), [0x37, 0xFF]);
        let req = TransferExitRequest::new(&[0xFF, 0xDD]);
        assert_eq!(req.serialize(), [0x37, 0xFF, 0xDD]);
    }

    #[test]
    fn deserializes_exit_response() {
        let res = TransferExitResponse::deserialize(&[0x77]);
        assert_eq!(res.payload, []);
        let res = TransferExitResponse::deserialize(&[0x77, 0xFF]);
        assert_eq!(res.payload, [0xFF]);
        let res = TransferExitResponse::deserialize(&[0x77, 0xFF, 0xDD]);
        assert_eq!(res.payload, [0xFF, 0xDD]);
    }
}
