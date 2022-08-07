use crate::{pdus, UdsClient, UdsError};

/// The different types of reset which can be done by the UDS server
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResetType {
    Hard,
    KeyOffOn,
    EnableRapidPowerShutdown,
    DisableRapidPowerShutdown,
    Other(u8),
}

impl From<u8> for ResetType {
    fn from(reset: u8) -> Self {
        match reset {
            0x01 => Self::Hard,
            0x02 => Self::KeyOffOn,
            0x03 => Self::EnableRapidPowerShutdown,
            0x04 => Self::DisableRapidPowerShutdown,
            reset => Self::Other(reset),
        }
    }
}

impl From<ResetType> for u8 {
    fn from(reset: ResetType) -> Self {
        match reset {
            ResetType::Hard => 0x01,
            ResetType::KeyOffOn => 0x02,
            ResetType::EnableRapidPowerShutdown => 0x03,
            ResetType::DisableRapidPowerShutdown => 0x04,
            ResetType::Other(reset) => reset,
        }
    }
}

impl UdsClient {
    pub async fn reset(&mut self, reset_type: impl Into<u8>) -> Result<(), UdsError> {
        let reset_type = reset_type.into();
        let req = pdus::ecu_reset::ResetRequest::new(reset_type);
        let res = self.query::<_, pdus::ecu_reset::ResetResponse>(req).await?;

        // Ensure we have the response for the reset type we actualy requested
        if res.reset_type != reset_type {
            return Err(UdsError::InvalidResponse(format!(
                "Expected response for reset type {} but got for {}",
                reset_type, res.reset_type
            )));
        }

        Ok(())
    }
}
