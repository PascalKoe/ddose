use super::{RxPdu, TxPdu};

pub const SID_SESSION_REQ: u8 = 0x10;
pub const SID_SESSION_RES: u8 = 0x50;

pub struct SessionRequest {
    session_type: u8,
}

impl SessionRequest {
    pub fn new(session_type: impl Into<u8>) -> Self {
        Self {
            session_type: session_type.into(),
        }
    }
}

impl TxPdu for SessionRequest {
    fn sid() -> u8 {
        SID_SESSION_REQ
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::from([SID_SESSION_REQ, self.session_type])
    }
}

pub struct SessionResponse {
    pub session_type: u8,
    pub p2: u16,
    pub p2_extended: u16,
}

impl RxPdu for SessionResponse {
    fn sid() -> u8 {
        SID_SESSION_RES
    }

    fn len_min() -> usize {
        6
    }

    fn len_max() -> usize {
        6
    }

    fn deserialize(data: &[u8]) -> Self {
        assert_eq!(data.len(), 6);
        assert_eq!(data[0], SID_SESSION_RES);

        let mut p2 = [0u8; 2];
        p2.copy_from_slice(&data[2..4]);
        let p2 = u16::from_be_bytes(p2);

        let mut p2_extended = [0u8; 2];
        p2_extended.copy_from_slice(&data[4..6]);
        let p2_extended = u16::from_be_bytes(p2_extended);

        Self {
            session_type: data[1],
            p2,
            p2_extended,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uds::pdus::{RxPdu, TxPdu};

    use super::{SessionRequest, SessionResponse};

    #[test]
    fn serializes_request() {
        let req = SessionRequest::new(0x01);
        assert_eq!(req.serialize(), [0x10, 0x01]);
        let req = SessionRequest::new(0x02);
        assert_eq!(req.serialize(), [0x10, 0x02]);
        let req = SessionRequest::new(0x03);
        assert_eq!(req.serialize(), [0x10, 0x03]);
    }

    #[test]
    fn deserializes_response() {
        let res = SessionResponse::deserialize(&[0x50, 0x01, 0xFF, 0xFF, 0x00, 0x00]);
        assert_eq!(res.session_type, 0x01);
        assert_eq!(res.p2, 0xFFFF);
        assert_eq!(res.p2_extended, 0x0000);
    }
}
