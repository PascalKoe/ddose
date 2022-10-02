use super::{RxPdu, TxPdu};

pub const SID_RESET_REQ: u8 = 0x11;
pub const SID_RESET_RES: u8 = 0x51;

pub struct ResetRequest {
    reset_type: u8,
}

impl ResetRequest {
    pub fn new(reset_type: impl Into<u8>) -> Self {
        Self {
            reset_type: reset_type.into(),
        }
    }
}

impl TxPdu for ResetRequest {
    fn sid() -> u8 {
        SID_RESET_REQ
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::from([SID_RESET_REQ, self.reset_type])
    }
}

pub struct ResetResponse {
    pub reset_type: u8,
}

impl RxPdu for ResetResponse {
    fn sid() -> u8 {
        SID_RESET_RES
    }

    fn len_min() -> usize {
        2
    }

    fn len_max() -> usize {
        2
    }

    fn deserialize(data: &[u8]) -> Self {
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], SID_RESET_RES);

        Self {
            reset_type: data[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uds::pdus::{RxPdu, TxPdu};

    use super::{ResetRequest, ResetResponse};

    #[test]
    fn serializes_request() {
        let req = ResetRequest::new(0x01);
        assert_eq!(req.serialize(), [0x11, 0x01]);
        let req = ResetRequest::new(0x02);
        assert_eq!(req.serialize(), [0x11, 0x02]);
        let req = ResetRequest::new(0x03);
        assert_eq!(req.serialize(), [0x11, 0x03]);
    }

    #[test]
    fn deserializes_response() {
        let res = ResetResponse::deserialize(&[0x51, 0x01]);
        assert_eq!(res.reset_type, 0x01);
        let res = ResetResponse::deserialize(&[0x51, 0x02]);
        assert_eq!(res.reset_type, 0x02);
        let res = ResetResponse::deserialize(&[0x51, 0x03]);
        assert_eq!(res.reset_type, 0x03);
    }
}
