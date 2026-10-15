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
    write_private(&path, &contents)?;
    Ok(())
}

/// Write `contents` to `path`, readable and writable by the owner only, from
/// the start: the file is created with mode 0600, never briefly with the
/// default permissions, and a file left with wider permissions (by an older
/// version) is narrowed before anything is written to it. Unix only; on
/// Windows, an ordinary write.
fn write_private(path: &std::path::Path, contents: &str) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)?;
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
        file.write_all(contents.as_bytes())
    }
    #[cfg(not(unix))]
    {
        fs::write(path, contents)
    }
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

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    /// A path of its own in the temporary directory, for one test.
    fn temp_path(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("todomate-cli-{}-{name}", std::process::id()));
        let _ = fs::remove_file(&path);
        path
    }

    fn mode(path: &std::path::Path) -> u32 {
        fs::metadata(path).unwrap().permissions().mode() & 0o777
    }

    #[test]
    fn new_file_is_private() {
        let path = temp_path("new");
        write_private(&path, "secret").unwrap();
        assert_eq!(mode(&path), 0o600);
        assert_eq!(fs::read_to_string(&path).unwrap(), "secret");
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn existing_file_is_narrowed_and_replaced() {
        let path = temp_path("existing");
        fs::write(&path, "an older, longer secret").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        write_private(&path, "new").unwrap();
        assert_eq!(mode(&path), 0o600);
        assert_eq!(fs::read_to_string(&path).unwrap(), "new");
        fs::remove_file(&path).unwrap();
    }
}
