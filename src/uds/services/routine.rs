use crate::uds::{pdus, UdsClient, UdsError};

#[derive(Debug, Clone, Copy)]
pub enum RoutineAction {
    Start,
    Stop,
    Result,
    Other(u8),
}

impl From<u8> for RoutineAction {
    fn from(id: u8) -> Self {
        match id {
            0x01 => Self::Start,
            0x02 => Self::Stop,
            0x03 => Self::Result,
            id => Self::Other(id),
        }
    }
}
impl From<RoutineAction> for u8 {
    fn from(action: RoutineAction) -> Self {
        match action {
            RoutineAction::Start => 0x01,
            RoutineAction::Stop => 0x02,
            RoutineAction::Result => 0x03,
            RoutineAction::Other(id) => id,
        }
    }
}

impl UdsClient {
    pub async fn control_routine(
        &mut self,
        action: RoutineAction,
        routine_id: impl Into<u16>,
        params: &[u8],
    ) -> Result<Vec<u8>, UdsError> {
        let routine_id = routine_id.into();
        let action: u8 = action.into();

        let req = pdus::routine::RoutineRequest::new(action, routine_id, params);
        let res = self.query::<_, pdus::routine::RoutineResponse>(req).await?;

        if res.control != action {
            return Err(UdsError::InvalidResponse(format!(
                "Expected response for control action 0x{:02X} but got {:02X}",
                action, res.control
            )));
        }

        if res.routine_id != routine_id {
            return Err(UdsError::InvalidResponse(format!(
                "Expected response for routine 0x{:04X} but got {:04X}",
                routine_id, res.routine_id
            )));
        }

        Ok(res.params)
    }
}
