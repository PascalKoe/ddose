use crate::uds::{pdus, UdsClient, UdsError};

impl UdsClient {
    pub async fn tester_present(&mut self) -> Result<(), UdsError> {
        let req = pdus::tester::TesterRequest::new();
        let _ = self.query::<_, pdus::tester::TesterResponse>(req).await?;
        Ok(())
    }
}
