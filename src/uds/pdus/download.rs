use super::{RxPdu, TxPdu};

pub const SID_DOWNLOAD_REQ: u8 = 0x34;
pub const SID_DOWNLOAD_RES: u8 = 0x74;

pub struct DownloadRequest {
    data_format: u8,
    addr_len_format: u8,
    memory_addr: u32,
    memory_size: u32,
}

impl DownloadRequest {
    pub fn new(data_format: u8, memory_addr: u32, memory_size: u32) -> Self {
        // TODO: Allow other lengths and sizes
        Self {
            data_format,
            addr_len_format: 0x44,
            memory_addr,
            memory_size,
        }
    }
}

impl TxPdu for DownloadRequest {
    fn sid() -> u8 {
        SID_DOWNLOAD_REQ
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::from([SID_DOWNLOAD_REQ, self.data_format, self.addr_len_format]);
        buffer.extend_from_slice(&self.memory_addr.to_be_bytes());
        buffer.extend_from_slice(&self.memory_size.to_be_bytes());
        buffer
    }
}

pub struct DownloadResponse {
    pub length_format_id: u8,
    pub block_len: u16,
}

impl RxPdu for DownloadResponse {
    fn sid() -> u8 {
        SID_DOWNLOAD_RES
    }

    fn len_min() -> usize {
        // TODO: Support other values than 2byte block length
        4
    }

    fn len_max() -> usize {
        // TODO: Support other values than 2byte block length
        4
    }

    fn deserialize(data: &[u8]) -> Self {
        assert_eq!(data.len(), 4);
        assert_eq!(data[0], SID_DOWNLOAD_RES);

        // Assert with have 2 bytes of block length
        // TODO: Replace with dynamic length
        assert_eq!(data[1], 0x20);

        let mut block_len = [0u8; 2];
        block_len.copy_from_slice(&data[2..4]);
        let block_len = u16::from_be_bytes(block_len);

        Self {
            length_format_id: data[1],
            block_len,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uds::pdus::{RxPdu, TxPdu};

    use super::{DownloadRequest, DownloadResponse};

    #[test]
    fn serializes_request() {
        let req = DownloadRequest::new(0x00, 0xFF00FF00, 0xDEADBEEF);
        assert_eq!(
            req.serialize(),
            [0x34, 0x00, 0x44, 0xFF, 0x00, 0xFF, 0x00, 0xDE, 0xAD, 0xBE, 0xEF]
        );
        let req = DownloadRequest::new(0x11, 0xFF00FF00, 0xDEADBEEF);
        assert_eq!(
            req.serialize(),
            [0x34, 0x11, 0x44, 0xFF, 0x00, 0xFF, 0x00, 0xDE, 0xAD, 0xBE, 0xEF]
        );
    }

    #[test]
    fn deserializes_response() {
        let res = DownloadResponse::deserialize(&[0x74, 0x20, 0x3F, 0xFF]);
        assert_eq!(res.length_format_id, 0x20);
        assert_eq!(res.block_len, 0x3FFF);
        let res = DownloadResponse::deserialize(&[0x74, 0x20, 0x00, 0x0F]);
        assert_eq!(res.length_format_id, 0x20);
        assert_eq!(res.block_len, 0x000F);
    }
}
