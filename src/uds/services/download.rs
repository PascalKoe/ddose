use std::num::Wrapping;

use crate::uds::{pdus, UdsClient, UdsError};

impl UdsClient {
    pub async fn download(&mut self, start_addr: u32, data: &[u8]) -> Result<(), UdsError> {
        // Currently only 32bits are supported
        assert!(data.len() < u32::MAX as usize);

        // Step 1: Start the download
        let dl_req = pdus::download::DownloadRequest::new(0x00, start_addr, data.len() as u32);
        let dl_res = self
            .query::<_, pdus::download::DownloadResponse>(dl_req)
            .await?;

        // Step 2: Transfer the data
        let data_block_len = dl_res.block_len - 15;
        let mut block_seq_counter = Wrapping(1u8);
        let blocks = data.chunks(data_block_len as usize);
        for block in blocks {
            let tr_req = pdus::transfer::TransferRequest::new(block_seq_counter.0, block);
            let _tr_res = self
                .query::<_, pdus::transfer::TransferResponse>(tr_req)
                .await?;
            block_seq_counter += 1;
        }

        // Step 3: Exit the transfer/download
        let ex_req = pdus::transfer::TransferExitRequest::new(&[]);
        let _eq_res = self
            .query::<_, pdus::transfer::TransferExitResponse>(ex_req)
            .await?;

        Ok(())
    }
}
