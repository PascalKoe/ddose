/// Negative response codes as defined in the UDS specification. The Negative response codes
/// get send by the server to the client in negative responses.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Nrc {
    PositiveResponse,
    GeneralReject,
    ServiceNotSupported,
    SubFunctionNotSupported,
    IncorrectMessageLengthOrInvalidFormat,
    ResponseTooLong,
    BusyRepeatReques,
    ConditionsNotCorrect,
    RequestSequenceError,
    RequestOutOfRange,
    SecurityAccessDenied,
    InvalidKey,
    ExceedNumberOfAttempts,
    RequiredTimeDelayNotExpired,
    UploadDownloadNotAccepted,
    TransferDataSuspended,
    GeneralProgrammingFailure,
    WrongBlockSequenceCounter,
    RequestCorrectlyReceivedResponsePending,
    SubFunctionNotSupportedInActiveSession,
    ServiceNotSupportedInActiveSession,

    Unknown(u8),
}

impl std::fmt::Display for Nrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nrc::PositiveResponse => write!(f, "Positive response"),
            Nrc::GeneralReject => write!(f, "General reject"),
            Nrc::ServiceNotSupported => write!(f, "Service not supported"),
            Nrc::SubFunctionNotSupported => {
                write!(f, "Sub-Function not supported")
            }
            Nrc::IncorrectMessageLengthOrInvalidFormat => {
                write!(f, "Incorrect message length or invalid format")
            }
            Nrc::ResponseTooLong => write!(f, "Response too long"),
            Nrc::BusyRepeatReques => write!(f, "Busy repeat reques"),
            Nrc::ConditionsNotCorrect => write!(f, "Conditions not correct"),
            Nrc::RequestSequenceError => write!(f, "Request sequence error"),
            Nrc::RequestOutOfRange => write!(f, "Request out of range"),
            Nrc::SecurityAccessDenied => write!(f, "Security access denied"),
            Nrc::InvalidKey => write!(f, "Invalid key"),
            Nrc::ExceedNumberOfAttempts => write!(f, "Exceed number of attempts"),
            Nrc::RequiredTimeDelayNotExpired => {
                write!(f, "requred time delay not expired")
            }
            Nrc::UploadDownloadNotAccepted => {
                write!(f, "Upload/Download not accepted")
            }
            Nrc::TransferDataSuspended => write!(f, "Transfer data suspended"),
            Nrc::GeneralProgrammingFailure => {
                write!(f, "General programming failure")
            }
            Nrc::WrongBlockSequenceCounter => {
                write!(f, "Wrong block sequence counter")
            }
            Nrc::RequestCorrectlyReceivedResponsePending => {
                write!(f, "Request correctly received, response pending")
            }
            Nrc::SubFunctionNotSupportedInActiveSession => {
                write!(f, "Sub-Function not supported in active session")
            }
            Nrc::ServiceNotSupportedInActiveSession => {
                write!(f, "Service not supported in active session")
            }
            Nrc::Unknown(nrc) => {
                write!(f, "Unknown NRC 0x{:02X}", nrc)
            }
        }
    }
}

impl From<u8> for Nrc {
    fn from(nrc: u8) -> Self {
        match nrc {
            0x00 => Self::PositiveResponse,
            0x10 => Self::GeneralReject,
            0x11 => Self::ServiceNotSupported,
            0x12 => Self::SubFunctionNotSupported,
            0x13 => Self::IncorrectMessageLengthOrInvalidFormat,
            0x14 => Self::ResponseTooLong,
            0x21 => Self::BusyRepeatReques,
            0x22 => Self::ConditionsNotCorrect,
            0x24 => Self::RequestSequenceError,
            0x31 => Self::RequestOutOfRange,
            0x33 => Self::SecurityAccessDenied,
            0x35 => Self::InvalidKey,
            0x36 => Self::ExceedNumberOfAttempts,
            0x37 => Self::RequiredTimeDelayNotExpired,
            0x70 => Self::UploadDownloadNotAccepted,
            0x71 => Self::TransferDataSuspended,
            0x72 => Self::GeneralProgrammingFailure,
            0x73 => Self::WrongBlockSequenceCounter,
            0x78 => Self::RequestCorrectlyReceivedResponsePending,
            0x7E => Self::SubFunctionNotSupportedInActiveSession,
            0x7F => Self::ServiceNotSupportedInActiveSession,
            nrc => Self::Unknown(nrc),
        }
    }
}

impl From<Nrc> for u8 {
    fn from(nrc: Nrc) -> Self {
        match nrc {
            Nrc::PositiveResponse => 0x00,
            Nrc::GeneralReject => 0x10,
            Nrc::ServiceNotSupported => 0x11,
            Nrc::SubFunctionNotSupported => 0x12,
            Nrc::IncorrectMessageLengthOrInvalidFormat => 0x13,
            Nrc::ResponseTooLong => 0x14,
            Nrc::BusyRepeatReques => 0x21,
            Nrc::ConditionsNotCorrect => 0x22,
            Nrc::RequestSequenceError => 0x24,
            Nrc::RequestOutOfRange => 0x31,
            Nrc::SecurityAccessDenied => 0x33,
            Nrc::InvalidKey => 0x35,
            Nrc::ExceedNumberOfAttempts => 0x36,
            Nrc::RequiredTimeDelayNotExpired => 0x37,
            Nrc::UploadDownloadNotAccepted => 0x70,
            Nrc::TransferDataSuspended => 0x71,
            Nrc::GeneralProgrammingFailure => 0x72,
            Nrc::WrongBlockSequenceCounter => 0x73,
            Nrc::RequestCorrectlyReceivedResponsePending => 0x78,
            Nrc::SubFunctionNotSupportedInActiveSession => 0x7E,
            Nrc::ServiceNotSupportedInActiveSession => 0x7F,
            Nrc::Unknown(nrc) => nrc,
        }
    }
}
