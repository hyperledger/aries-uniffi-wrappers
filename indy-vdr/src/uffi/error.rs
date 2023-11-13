use indy_vdr::common::error::{VdrError, VdrErrorKind};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone, uniffi::Error, thiserror::Error)]
pub enum ErrorCode {
    Config { message: String },
    Connection { message: String },
    FileSystem { message: String },
    Input { message: String },
    Resource { message: String },
    Unavailable { message: String },
    Unexpected { message: String },
    Incompatible { message: String },
    PoolNoConsensus { message: String },
    PoolRequestFailed { message: String },
    PoolTimeout { message: String },
    Resolver { message: String },
    Success {},
}

impl From<VdrError> for ErrorCode {
    fn from(err: VdrError) -> ErrorCode {
        match err.kind() {
            VdrErrorKind::Config => ErrorCode::Config {
                message: err.to_string(),
            },
            VdrErrorKind::Connection => ErrorCode::Connection {
                message: err.to_string(),
            },
            VdrErrorKind::FileSystem(_) => ErrorCode::FileSystem {
                message: err.to_string(),
            },
            VdrErrorKind::Input => ErrorCode::Input {
                message: err.to_string(),
            },
            VdrErrorKind::Resource => ErrorCode::Resource {
                message: err.to_string(),
            },
            VdrErrorKind::Unavailable => ErrorCode::Unavailable {
                message: err.to_string(),
            },
            VdrErrorKind::Unexpected => ErrorCode::Unexpected {
                message: err.to_string(),
            },
            VdrErrorKind::Incompatible => ErrorCode::Incompatible {
                message: err.to_string(),
            },
            VdrErrorKind::PoolNoConsensus => ErrorCode::PoolNoConsensus {
                message: err.to_string(),
            },
            VdrErrorKind::PoolRequestFailed(_) => ErrorCode::PoolRequestFailed {
                message: err.to_string(),
            },
            VdrErrorKind::PoolTimeout => ErrorCode::PoolTimeout {
                message: err.to_string(),
            },
            VdrErrorKind::Resolver => ErrorCode::Resolver {
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

pub fn input_err<M>(msg: M) -> ErrorCode
where
    M: fmt::Display + Send + Sync + 'static,
{
    ErrorCode::Input {
        message: msg.to_string(),
    }
}
