use crate::{pdus, UdsClient, UdsError};

/// Represents the different sessions defined in the UDS specification.
/// Non standard sessions can be represented using [`Session::Other(nrc)`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionType {
    Default,
    Programming,
    Extended,
    SafetySystem,
    Other(u8),
}

impl From<u8> for SessionType {
    fn from(st: u8) -> Self {
        match st {
            0x01 => SessionType::Default,
            0x02 => SessionType::Programming,
            0x03 => SessionType::Extended,
            0x04 => SessionType::SafetySystem,
            st => SessionType::Other(st),
        }
    }
}

impl From<SessionType> for u8 {
    fn from(session: SessionType) -> Self {
        match session {
            SessionType::Default => 0x01,
            SessionType::Programming => 0x02,
            SessionType::Extended => 0x03,
            SessionType::SafetySystem => 0x04,
            SessionType::Other(st) => st,
        }
    }
}

impl UdsClient {
    pub async fn start_session(
        &mut self,
        session_id: impl Into<u8>,
    ) -> Result<(std::time::Duration, std::time::Duration), UdsError> {
        let session_id = session_id.into();
        let req_pdu = pdus::dsc::SessionRequest::new(session_id);
        let res_pdu = self.query::<_, pdus::dsc::SessionResponse>(req_pdu).await?;

        // Ensure we actually entered the session, we requested
        if res_pdu.session_type != session_id {
            return Err(UdsError::Other(format!(
                "Requested session id 0x{:02X} but received response for session id 0x{:02X}",
                session_id, res_pdu.session_type
            )));
        }

        // The p2 timing is represendet in millis, the p2* als 10ms per integer step
        let p2 = std::time::Duration::from_millis(res_pdu.p2 as u64);
        let p2_extended = std::time::Duration::from_millis(res_pdu.p2_extended as u64 * 10);

        Ok((p2, p2_extended))
    }
}
