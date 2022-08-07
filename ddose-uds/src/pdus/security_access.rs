use super::{RxPdu, TxPdu};

const SID_SEC_REQ: u8 = 0x27;
const SID_SEC_RES: u8 = 0x67;

pub struct SeedRequest<'a> {
    sec_level: u8,
    data: &'a [u8],
}

impl<'a> SeedRequest<'a> {
    pub fn new(sec_level: impl Into<u8>, data: &'a [u8]) -> Self {
        Self {
            sec_level: sec_level.into(),
            data,
        }
    }
}

impl<'a> TxPdu for SeedRequest<'a> {
    fn sid() -> u8 {
        SID_SEC_REQ
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![SID_SEC_REQ, self.sec_level];
        buffer.extend_from_slice(self.data);
        buffer
    }
}

pub struct SeedResponse {
    pub sec_level: u8,
    pub seed: Vec<u8>,
}

impl RxPdu for SeedResponse {
    fn sid() -> u8 {
        SID_SEC_RES
    }

    fn len_min() -> usize {
        2
    }

    fn len_max() -> usize {
        usize::MAX
    }

    fn deserialize(data: &[u8]) -> Self {
        assert!(data.len() >= 2);
        assert_eq!(data[0], SID_SEC_RES);

        let mut seed = Vec::new();
        seed.extend_from_slice(&data[2..]);

        Self {
            sec_level: data[1],
            seed,
        }
    }
}

pub struct KeyRequest<'a> {
    sec_level: u8,
    key: &'a [u8],
}

impl<'a> KeyRequest<'a> {
    pub fn new(sec_level: impl Into<u8>, key: &'a [u8]) -> Self {
        Self {
            sec_level: sec_level.into(),
            key,
        }
    }
}

impl<'a> TxPdu for KeyRequest<'a> {
    fn sid() -> u8 {
        SID_SEC_REQ
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![SID_SEC_REQ, self.sec_level];
        buffer.extend_from_slice(self.key);

        buffer
    }
}

pub struct KeyResponse {
    pub sec_level: u8,
}

impl RxPdu for KeyResponse {
    fn sid() -> u8 {
        SID_SEC_RES
    }

    fn len_min() -> usize {
        2
    }

    fn len_max() -> usize {
        2
    }

    fn deserialize(data: &[u8]) -> Self {
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], SID_SEC_RES);

        Self { sec_level: data[1] }
    }
}

#[cfg(test)]
mod tests {
    use crate::pdus::{RxPdu, TxPdu};

    use super::{KeyRequest, KeyResponse, SeedRequest, SeedResponse};

    #[test]
    fn serializes_seed_req_without_data() {
        let req = SeedRequest::new(0x00, &[]);
        assert_eq!(req.serialize(), [0x27, 0x00]);
        let req = SeedRequest::new(0x01, &[]);
        assert_eq!(req.serialize(), [0x27, 0x01]);
        let req = SeedRequest::new(0x02, &[]);
        assert_eq!(req.serialize(), [0x27, 0x02]);
    }

    #[test]
    fn serializes_seed_req_with_data() {
        let req = SeedRequest::new(0x00, &[0xFF]);
        assert_eq!(req.serialize(), [0x27, 0x00, 0xFF]);
        let req = SeedRequest::new(0x01, &[0xEE, 0xFF]);
        assert_eq!(req.serialize(), [0x27, 0x01, 0xEE, 0xFF]);
        let req = SeedRequest::new(0x02, &[0x00, 0xFF, 0x00]);
        assert_eq!(req.serialize(), [0x27, 0x02, 0x00, 0xFF, 0x00]);
    }

    #[test]
    fn deserializes_seed_res_without_seed() {
        let res = SeedResponse::deserialize(&[0x67, 0x01]);
        assert_eq!(res.sec_level, 0x01);
        assert_eq!(res.seed, []);

        let res = SeedResponse::deserialize(&[0x67, 0x02]);
        assert_eq!(res.sec_level, 0x02);
        assert_eq!(res.seed, []);

        let res = SeedResponse::deserialize(&[0x67, 0x03]);
        assert_eq!(res.sec_level, 0x03);
        assert_eq!(res.seed, []);
    }

    #[test]
    fn deserializes_seed_res_with_seed() {
        let res = SeedResponse::deserialize(&[0x67, 0x01, 0xFF]);
        assert_eq!(res.sec_level, 0x01);
        assert_eq!(res.seed, [0xFF]);

        let res = SeedResponse::deserialize(&[0x67, 0x02, 0xFF, 0xFF]);
        assert_eq!(res.sec_level, 0x02);
        assert_eq!(res.seed, [0xFF, 0xFF]);

        let res = SeedResponse::deserialize(&[0x67, 0x03, 0xFF, 0xFF, 0xFF]);
        assert_eq!(res.sec_level, 0x03);
        assert_eq!(res.seed, [0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn serializes_key_req() {
        let req = KeyRequest::new(0x02, &[]);
        assert_eq!(req.serialize(), [0x27, 0x02]);
        let req = KeyRequest::new(0x02, &[0xFF]);
        assert_eq!(req.serialize(), [0x27, 0x02, 0xFF]);
        let req = KeyRequest::new(0x02, &[0xFF, 0xEE]);
        assert_eq!(req.serialize(), [0x27, 0x02, 0xFF, 0xEE]);
        let req = KeyRequest::new(0x02, &[0xFF, 0xEE, 0xDD]);
        assert_eq!(req.serialize(), [0x27, 0x02, 0xFF, 0xEE, 0xDD]);
    }

    #[test]
    fn deserializes_key_res() {
        let res = KeyResponse::deserialize(&[0x67, 0x02]);
        assert_eq!(res.sec_level, 0x02);
        let res = KeyResponse::deserialize(&[0x67, 0x04]);
        assert_eq!(res.sec_level, 0x04);
        let res = KeyResponse::deserialize(&[0x67, 0x06]);
        assert_eq!(res.sec_level, 0x06);
    }
}
