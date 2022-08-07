use ddose_isotp::IsotpConnection;
use nrc::Nrc;
use pdus::{RxPdu, TxPdu};
use thiserror::Error;

mod nrc;
mod pdus;
mod services;

pub use nrc::*;
pub use services::*;

#[derive(Debug, Error)]
pub enum UdsError {
    #[error("Isotp Error: {0}")]
    TransportError(#[from] std::io::Error),

    #[error("Received NRC: {0}")]
    NegativeResponse(Nrc),

    #[error("Received invalid response: {0}")]
    InvalidResponse(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("{0}")]
    Other(String),
}

pub struct UdsClient {
    isotp_conn: IsotpConnection,
}

impl UdsClient {
    pub fn new(isotp_conn: IsotpConnection) -> Self {
        Self { isotp_conn }
    }

    pub async fn query<Req, Res>(&mut self, req: Req) -> Result<Res, UdsError>
    where
        Req: TxPdu,
        Res: RxPdu,
    {
        const RESPONSE_SID_NEGATIVE: u8 = 0x7F;

        let data = req.serialize();
        self.isotp_conn.write(&data).await?;

        // We only need to send the request once but we use the loop the continue receiving when the
        // server needs more time to answer
        loop {
            // TODO: add read timeout (p2 or p2_extended)
            let mut buffer = [0; 4096];
            let bytes_read = self.isotp_conn.read(&mut buffer).await?;
            let data = &buffer[..bytes_read];

            // Handle all the negative responses
            if data[0] == RESPONSE_SID_NEGATIVE {
                // Negative Response: NR SID NRC
                if data.len() != 3 {
                    return Err(UdsError::InvalidResponse(format!(
                        "Response is negative and has invalid length '{}'. Expected length of '3'",
                        data.len()
                    )));
                }

                // For negative responses, the original NRC is echoed back, not the response SID
                if data[1] != Req::sid() {
                    return Err(UdsError::InvalidResponse(
                        "Received response for another SID".to_string(),
                    ));
                }

                // UDS allows the server to send out the NRC 0x78 which signals that more time is
                // needed for the requested operation. Therefor we just wait for the next response.
                if data[2] == Nrc::RequestCorrectlyReceivedResponsePending.into() {
                    continue;
                }

                // At this point we have some negative response that we should return the the consumer
                // of the UDS client
                return Err(UdsError::NegativeResponse(data[2].into()));
            }

            // At this point we only have positive responses
            // We can do the following basics checks to ensure we have a parsable PDU
            //  - Check that the PDU is actually the one we expect by checking the SID
            //  - Ensure that the PDU has an acceptable length
            if data[0] != Res::sid() {
                return Err(UdsError::InvalidResponse(format!(
                    "Expected the SID {:02X} but received SID {:02X}",
                    Res::sid(),
                    data[0]
                )));
            }

            if data.len() < Res::len_min() || data.len() > Res::len_max() {
                return Err(UdsError::InvalidResponse(format!(
                    "Expected the length of the response to be between {} and {} but got length of {}",
                    Res::len_min(),
                    Res::len_max(),
                    data.len()
                )));
            }

            // We checked all we can, so just deserialize it
            let response = Res::deserialize(data);
            return Ok(response);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
