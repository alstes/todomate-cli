use crate::error::CliError;
use anyhow::Result;
use keyring::Entry;
use std::fs;

const SERVICE: &str = "todo.ac";
const KEY_JWT: &str = "jwt";
const KEY_REFRESH: &str = "refresh_token";
const KEY_SUBSCRIPTION: &str = "subscription_key";

pub struct Credentials {
    pub jwt: String,
    pub refresh_token: String,
    pub subscription_key: String,
}

pub fn save(creds: &Credentials) -> Result<()> {
    match try_keychain_save(creds) {
        Ok(()) => Ok(()),
        Err(_) => {
            eprintln!("warning: keychain unavailable, falling back to plaintext credentials file");
            save_to_file(creds)
        }
    }
}

pub fn load() -> Result<Credentials> {
    match try_keychain_load() {
        Ok(creds) => Ok(creds),
        Err(_) => load_from_file(),
    }
}

pub fn delete() -> Result<()> {
    // Try both — ignore individual errors so partial state is always cleaned up.
    let _ = try_keychain_delete();
    let _ = delete_file();
    Ok(())
}

// --- keychain ---

fn try_keychain_save(creds: &Credentials) -> Result<()> {
    Entry::new(SERVICE, KEY_JWT)?.set_password(&creds.jwt)?;
    Entry::new(SERVICE, KEY_REFRESH)?.set_password(&creds.refresh_token)?;
    Entry::new(SERVICE, KEY_SUBSCRIPTION)?.set_password(&creds.subscription_key)?;
    // Verify the write actually persisted — on macOS the first write to a new
    // service can silently succeed without storing anything.
    Entry::new(SERVICE, KEY_JWT)?.get_password()?;
    Ok(())
}

fn try_keychain_load() -> Result<Credentials> {
    let jwt = Entry::new(SERVICE, KEY_JWT)?.get_password()?;
    let refresh_token = Entry::new(SERVICE, KEY_REFRESH)?.get_password()?;
    let subscription_key = Entry::new(SERVICE, KEY_SUBSCRIPTION)?.get_password()?;
    Ok(Credentials {
        jwt,
        refresh_token,
        subscription_key,
    })
}

fn try_keychain_delete() -> Result<()> {
    Entry::new(SERVICE, KEY_JWT)?.delete_credential()?;
    Entry::new(SERVICE, KEY_REFRESH)?.delete_credential()?;
    Entry::new(SERVICE, KEY_SUBSCRIPTION)?.delete_credential()?;
    Ok(())
}

// --- plaintext file fallback ---

#[derive(serde::Serialize, serde::Deserialize)]
struct CredentialsFile {
    jwt: String,
    refresh_token: String,
    subscription_key: String,
}

fn creds_path() -> Result<std::path::PathBuf> {
    let dir = dirs::config_dir()
        .ok_or_else(|| CliError::Config("Cannot determine config directory".to_string()))?
        .join("todomate");
    Ok(dir.join("credentials.toml"))
}

fn save_to_file(creds: &Credentials) -> Result<()> {
    let path = creds_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let file = CredentialsFile {
        jwt: creds.jwt.clone(),
        refresh_token: creds.refresh_token.clone(),
        subscription_key: creds.subscription_key.clone(),
    };
    let contents = toml::to_string_pretty(&file).map_err(|e| CliError::Config(e.to_string()))?;
    fs::write(&path, &contents)?;
    // Restrict permissions to owner-only (Unix only — on Windows this is a no-op)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

fn load_from_file() -> Result<Credentials> {
    let path = creds_path()?;
    if !path.exists() {
        return Err(CliError::NotAuthenticated.into());
    }
    let contents = fs::read_to_string(&path)?;
    let file: CredentialsFile =
        toml::from_str(&contents).map_err(|e| CliError::Config(e.to_string()))?;
    Ok(Credentials {
        jwt: file.jwt,
        refresh_token: file.refresh_token,
        subscription_key: file.subscription_key,
    })
}

fn delete_file() -> Result<()> {
    let path = creds_path()?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
