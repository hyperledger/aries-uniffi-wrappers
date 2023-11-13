use anoncreds::{Error, ErrorKind};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone, uniffi::Error, thiserror::Error)]
pub enum ErrorCode {
    Input { message: String },
    IOError { message: String },
    InvalidState { message: String },
    Unexpected { message: String },
    CredentialRevoked { message: String },
    InvalidUserRevocId { message: String },
    ProofRejected { message: String },
    RevocationRegistryFull { message: String },
}

impl From<Error> for ErrorCode {
    fn from(err: Error) -> ErrorCode {
        match err.kind() {
            ErrorKind::Input => ErrorCode::Input {
                message: err.to_string(),
            },
            ErrorKind::IOError => ErrorCode::IOError {
                message: err.to_string(),
            },
            ErrorKind::InvalidState => ErrorCode::InvalidState {
                message: err.to_string(),
            },
            ErrorKind::Unexpected => ErrorCode::Unexpected {
                message: err.to_string(),
            },
            ErrorKind::CredentialRevoked => ErrorCode::CredentialRevoked {
                message: err.to_string(),
            },
            ErrorKind::InvalidUserRevocId => ErrorCode::InvalidUserRevocId {
                message: err.to_string(),
            },
            ErrorKind::ProofRejected => ErrorCode::ProofRejected {
                message: err.to_string(),
            },
            ErrorKind::RevocationRegistryFull => ErrorCode::RevocationRegistryFull {
                message: err.to_string(),
            },
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<serde_json::Error> for ErrorCode {
    fn from(err: serde_json::Error) -> Self {
        ErrorCode::Input {
            message: err.to_string(),
        }
    }
}
