use crate::{cmd::errors::AppError, env_vars};
use keyring::Entry;

pub trait HandleCredentials {
    fn set_dracoon_env(&self, secret: &str) -> Result<(), AppError>;
    fn get_dracoon_env(&self) -> Result<String, AppError>;
    fn delete_dracoon_env(&self) -> Result<(), AppError>;
}

impl HandleCredentials for Entry {
    fn set_dracoon_env(&self, secret: &str) -> Result<(), AppError> {
        match self.set_password(secret) {
            Ok(()) => Ok(()),
            Err(_) => Err(AppError::CredentialStorageFailed),
        }
    }
    fn get_dracoon_env(&self) -> Result<String, AppError> {
        match self.get_password() {
            Ok(pwd) => Ok(pwd),
            Err(_) => Err(AppError::InvalidAccount),
        }
    }
    fn delete_dracoon_env(&self) -> Result<(), AppError> {
        if self.get_password().is_err() {
            return Err(AppError::InvalidAccount);
        }

        match self.delete_credential() {
            Ok(()) => Ok(()),
            Err(_) => Err(AppError::CredentialDeletionFailed),
        }
    }
}

pub fn get_client_credentials() -> (String, String) {
    let client_id = env_vars::CLIENT_ID.to_string();
    let client_secret = env_vars::CLIENT_SECRET.to_string();

    (client_id, client_secret)
}
