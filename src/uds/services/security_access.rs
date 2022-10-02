use crate::uds::{pdus, UdsClient, UdsError};

impl UdsClient {
    pub async fn unlock(
        &mut self,
        sec_level: impl Into<u8>,
        seed_data: &[u8],
        key_algo: impl FnOnce(&[u8]) -> Result<Vec<u8>, String>,
    ) -> Result<(), UdsError> {
        // The unlock security level must always be an odd number as
        // the key request is the next even number (defined in standard)
        let sec_level = sec_level.into();
        if sec_level % 2 != 1 {
            return Err(UdsError::InvalidRequest(format!(
                "The security level must be an even number but is {}",
                sec_level
            )));
        }

        let seed_req = pdus::security_access::SeedRequest::new(sec_level, seed_data);
        let seed_res = self
            .query::<_, pdus::security_access::SeedResponse>(seed_req)
            .await?;

        // Ensure we have the response for the security level we actualy requested
        if seed_res.sec_level != sec_level {
            return Err(UdsError::InvalidResponse(format!(
                "Expected response for security level {} but got for {}",
                sec_level, seed_res.sec_level
            )));
        }

        let key = key_algo(&seed_res.seed)
            .map_err(|e| UdsError::Other(format!("Failed to generate key: {}", e)))?;

        let key_sec_level = sec_level + 1;
        let key_req = pdus::security_access::KeyRequest::new(key_sec_level, &key);
        let key_res = self
            .query::<_, pdus::security_access::KeyResponse>(key_req)
            .await?;

        // Ensure we have the response for the security level we actualy requested
        if key_res.sec_level != key_sec_level {
            return Err(UdsError::InvalidResponse(format!(
                "Expected response for security level {} but got for {}",
                key_sec_level, key_res.sec_level
            )));
        }

        Ok(())
    }
}
