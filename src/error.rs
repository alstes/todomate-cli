use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Not logged in. Run `todo auth login` first.")]
    NotAuthenticated,

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("API error {status}: {message}")]
    ApiError { status: u16, message: String },

    #[error("Token refresh failed — please run `todo auth login` again.")]
    RefreshFailed,

    #[error("Keychain error: {0}")]
    Keychain(#[from] keyring::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[allow(dead_code)]
    #[error("{0}")]
    Other(String),
}
