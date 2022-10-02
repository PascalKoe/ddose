use super::{RxPdu, TxPdu};

pub const SID_ROUTINE_REQ: u8 = 0x31;
pub const SID_ROUTINE_RES: u8 = 0x71;

pub struct RoutineRequest<'a> {
    control: u8,
    routine_id: u16,
    params: &'a [u8],
}

impl<'a> RoutineRequest<'a> {
    pub fn new(control: impl Into<u8>, routine_id: impl Into<u16>, params: &'a [u8]) -> Self {
        Self {
            control: control.into(),
            routine_id: routine_id.into(),
            params,
        }
    }
}

impl<'a> TxPdu for RoutineRequest<'a> {
    fn sid() -> u8 {
        SID_ROUTINE_REQ
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::from([SID_ROUTINE_REQ, self.control]);
        buffer.extend_from_slice(&self.routine_id.to_be_bytes());
        buffer.extend_from_slice(self.params);
        buffer
    }
}

pub struct RoutineResponse {
    pub control: u8,
    pub routine_id: u16,
    pub info: u8,
    pub params: Vec<u8>,
}

impl RxPdu for RoutineResponse {
    fn sid() -> u8 {
        SID_ROUTINE_RES
    }

    fn len_min() -> usize {
        5
    }

    fn len_max() -> usize {
        usize::MAX
    }

    fn deserialize(data: &[u8]) -> Self {
        assert!(data.len() >= Self::len_min());
        assert_eq!(data[0], SID_ROUTINE_RES);

        let mut routine_id = [0u8; 2];
        routine_id.copy_from_slice(&data[2..4]);
        let routine_id = u16::from_be_bytes(routine_id);

        let params = Vec::from(&data[5..]);

        Self {
            control: data[1],
            routine_id,
            info: data[4],
            params,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uds::pdus::{RxPdu, TxPdu};

    use super::{RoutineRequest, RoutineResponse};

    #[test]
    fn serializes_request() {
        let req = RoutineRequest::new(0x01, 0xFF00u16, &[]);
        assert_eq!(req.serialize(), [0x31, 0x01, 0xFF, 0x00]);
        let req = RoutineRequest::new(0x02, 0x00FFu16, &[0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(
            req.serialize(),
            [0x31, 0x02, 0x00, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF]
        );
    }

    #[test]
    fn deserializes_response() {
        let res = RoutineResponse::deserialize(&[0x71, 0x01, 0x00, 0xFF, 0x00]);
        assert_eq!(res.control, 0x01);
        assert_eq!(res.routine_id, 0x00FF);
        assert_eq!(res.info, 0x00);
        assert_eq!(res.params, []);
        let res = RoutineResponse::deserialize(&[0x71, 0x01, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF]);
        assert_eq!(res.control, 0x01);
        assert_eq!(res.routine_id, 0x00FF);
        assert_eq!(res.info, 0x00);
        assert_eq!(res.params, [0xFF, 0xFF, 0xFF]);
    }
}
