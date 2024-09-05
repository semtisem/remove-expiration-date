use console::Term;
use keyring::Entry;
use tracing::error;

use self::{
    config::credentials::{get_client_credentials, HandleCredentials},
    errors::AppError,
    models::PasswordAuth,
    utils::strings::format_error_message,
};
use dco3::{
    auth::{Connected, Disconnected, OAuth2Flow},
    Dracoon, DracoonBuilder,
};

pub mod config;
pub mod errors;
pub mod models;
pub mod remover;
pub mod utils;

// service name to store
const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");

async fn init_dracoon(
    url_path: &str,
    password_auth: Option<PasswordAuth>,
    is_transfer: bool,
) -> Result<Dracoon<Connected>, AppError> {
    let (client_id, client_secret) = get_client_credentials();
    let base_url = parse_base_url(url_path.to_string())?;

    // use multiple access tokens for transfers
    let token_rotation = if is_transfer { 5 } else { 1 };

    let syncoon_user_agent = format!("{}|{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let dracoon = DracoonBuilder::new()
        .with_base_url(base_url.clone())
        .with_client_id(client_id)
        .with_client_secret(client_secret)
        .with_token_rotation(token_rotation)
        .with_user_agent(syncoon_user_agent)
        .build()?;

    let entry = Entry::new(SERVICE_NAME, &base_url).map_err(|_| AppError::CredentialStorageFailed);

    // Always use password auth first if present
    if let Some(password_auth) = password_auth {
        return authenticate_password_flow(dracoon, password_auth).await;
    }
    // Entry not present & no password auth? Game over.
    let Ok(entry) = entry else {
        error!("Can't open keyring entry for {}", base_url);
        return Err(AppError::CredentialStorageFailed);
    };

    // Attempt to use refresh token if exists
    if let Ok(refresh_token) = entry.get_dracoon_env() {
        if let Ok(dracoon) = dracoon
            .clone()
            .connect(OAuth2Flow::RefreshToken(refresh_token))
            .await
        {
            return Ok(dracoon);
        }
        // Refresh token didn't work, delete it
        let _ = entry.delete_dracoon_env();
    }

    // Final resort: auth code flow
    authenticate_auth_code_flow(dracoon, entry).await
}

async fn authenticate_auth_code_flow(
    dracoon: Dracoon<Disconnected>,
    entry: Entry,
) -> Result<Dracoon<Connected>, AppError> {
    println!("Please log in via browser (open url): ");
    println!("{}", dracoon.get_authorize_url());

    let auth_code = dialoguer::Password::new()
        .with_prompt("Please enter authorization code")
        .interact()
        .or(Err(AppError::IoError))?;

    let dracoon = dracoon
        .connect(OAuth2Flow::AuthCodeFlow(auth_code.trim_end().into()))
        .await?;

    // TODO: if this fails, offer to store in plain
    let res = entry.set_dracoon_env(&dracoon.get_refresh_token().await);
    match res {
        Ok(_) => Ok(dracoon),
        Err(_) => {
            error!("Failed to store refresh token in keyring.");
            Ok(dracoon)
        }
    }
}

async fn authenticate_password_flow(
    dracoon: Dracoon<Disconnected>,
    password_auth: PasswordAuth,
) -> Result<Dracoon<Connected>, AppError> {
    let dracoon = dracoon
        .connect(OAuth2Flow::password_flow(password_auth.0, password_auth.1))
        .await?;

    Ok(dracoon)
}

fn parse_base_url(url_str: String) -> Result<String, AppError> {
    if url_str.starts_with("http://") {
        error!("HTTP is not supported.");
        return Err(AppError::InvalidUrl(url_str));
    };

    let url_str = if url_str.starts_with("https://") {
        url_str
    } else {
        format!("https://{url_str}")
    };

    let uri_fragments: Vec<&str> = url_str[8..].split('/').collect();

    match uri_fragments.len() {
        2.. => Ok(format!("https://{}", uri_fragments[0])),
        _ => Err(AppError::InvalidUrl(url_str)),
    }
}

pub fn handle_error(term: &Term, err: &AppError) {
    let err_msg = get_error_message(err);
    let err_msg = format_error_message(&err_msg);

    error!("{}", err_msg);

    term.write_line(&err_msg)
        .expect("Error writing error message to terminal.");

    // exit with error code
    std::process::exit(1);
}

fn get_error_message(err: &AppError) -> String {
    match err {
        AppError::InvalidUrl(url) => format!("Invalid URL: {url}"),
        AppError::IoError => "Error reading / writing content.".into(),
        AppError::DracoonError(e) => format!("{e}"),
        AppError::ConnectionFailed => "Connection failed.".into(),
        AppError::CredentialDeletionFailed => "Credential deletion failed.".into(),
        AppError::CredentialStorageFailed => "Credential store failed.".into(),
        AppError::InvalidAccount => "Invalid account.".into(),
        AppError::Unknown => "Unknown error.".into(),
        AppError::DracoonS3Error(e) => format!("{e}"),
        AppError::DracoonAuthError(e) => format!("{e}"),
        AppError::InvalidArgument(msg) => msg.to_string(),
        AppError::LogFileCreationFailed => "Log file creation failed.".into(),
    }
}

pub fn print_version(term: &Term) -> Result<(), AppError> {
    term.write_line(get_version().as_str())
        .map_err(|_| AppError::IoError)
}

pub fn get_version() -> String {
    format!(
        "🚀 remove-expiration-date {}\n▶︎ https://github.com/semtisem/remove-expiration-date",
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_url_parse_https() {
        let base_url = parse_base_url("https://bla.dracoon.com/bla/somefile.pdf".into()).unwrap();
        assert_eq!(base_url, "https://bla.dracoon.com");
    }

    #[test]
    fn test_base_url_parse_no_https() {
        let base_url = parse_base_url("bla.dracoon.com/bla/somefile.pdf".into()).unwrap();
        assert_eq!(base_url, "https://bla.dracoon.com");
    }

    #[test]
    fn test_base_url_parse_invalid_path() {
        let base_url = parse_base_url("bla.dracoon.com".into());
        assert_eq!(
            base_url,
            Err(AppError::InvalidUrl("https://bla.dracoon.com".into()))
        );
    }
}
