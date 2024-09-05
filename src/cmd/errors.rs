use dco3::{
    auth::models::{DracoonAuthErrorResponse, DracoonErrorResponse},
    nodes::S3ErrorResponse,
    DracoonClientError,
};
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum AppError {
    #[error("Connection to DRACOON failed")]
    ConnectionFailed,
    #[error("Unknown error")]
    Unknown,
    #[error("Invalid DRACOON url format")]
    InvalidUrl(String),
    #[error("Saving DRACOON credentials failed")]
    CredentialStorageFailed,
    #[error("Deleting DRACOON credentials failed")]
    CredentialDeletionFailed,
    #[error("DRACOON account not found")]
    InvalidAccount,
    #[error("DRACOON HTTP API error")]
    DracoonError(DracoonErrorResponse),
    #[error("DRACOON HTTP S3 error")]
    DracoonS3Error(Box<S3ErrorResponse>),
    #[error("DRACOON HTTP authentication error")]
    DracoonAuthError(DracoonAuthErrorResponse),
    #[error("IO error")]
    IoError,
    #[error("Invalid argument")]
    InvalidArgument(String),
    #[error("Log file creation failed")]
    LogFileCreationFailed,
}

impl From<DracoonClientError> for AppError {
    fn from(value: DracoonClientError) -> Self {
        match value {
            DracoonClientError::ConnectionFailed(_) => AppError::ConnectionFailed,
            DracoonClientError::Http(err) => AppError::DracoonError(err),
            DracoonClientError::Auth(err) => AppError::DracoonAuthError(err),
            DracoonClientError::InvalidUrl(url) => AppError::InvalidUrl(url),
            DracoonClientError::IoError => AppError::IoError,
            DracoonClientError::S3Error(err) => AppError::DracoonS3Error(err),
            DracoonClientError::MissingArgument => {
                AppError::InvalidArgument("Missing argument (password set?)".to_string())
            }
            DracoonClientError::CryptoError(_) => {
                AppError::InvalidArgument(("Wrong encryption secret.").to_string())
            }
            _ => AppError::Unknown,
        }
    }
}
