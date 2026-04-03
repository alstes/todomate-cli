pub mod device_flow;
pub mod gh_token;
pub mod token_store;

use crate::api::ApiClient;
use crate::cli::AuthCommand;
use crate::error::CliError;
use anyhow::Result;
use token_store::Credentials;

pub fn handle(action: &AuthCommand, api_base_url: &str) -> Result<()> {
    match action {
        AuthCommand::Login { device_flow } => login(*device_flow, api_base_url),
        AuthCommand::Logout => logout(),
        AuthCommand::Status => status(),
    }
}

fn login(device_flow: bool, api_base_url: &str) -> Result<()> {
    if device_flow {
        eprintln!("Device Flow is not yet implemented. See Phase 8.");
        std::process::exit(1);
    }

    let github_token = gh_token::get_token()?;

    eprint!("Enter your API key (from todo.ac → Settings → API Access): ");
    let subscription_key = read_line()?;
    if subscription_key.is_empty() {
        return Err(CliError::AuthFailed("API key cannot be empty.".to_string()).into());
    }

    let client = ApiClient::new(api_base_url.to_string());
    let auth_resp = client.exchange_github_token(&github_token, &subscription_key)?;

    token_store::save(&Credentials {
        jwt: auth_resp.access_token,
        refresh_token: auth_resp.refresh_token,
        subscription_key,
    })?;

    println!("Logged in as @{}", auth_resp.user.login);
    Ok(())
}

fn logout() -> Result<()> {
    token_store::delete()?;
    println!("Logged out.");
    Ok(())
}

fn status() -> Result<()> {
    let creds = token_store::load()?;
    match decode_jwt_claims(&creds.jwt) {
        Some((github_user, exp)) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            if exp <= now {
                println!("Logged in as @{github_user} (token expired — run `todo auth login`)");
            } else {
                let remaining = exp - now;
                let h = remaining / 3600;
                let m = (remaining % 3600) / 60;
                println!("Logged in as @{github_user}, token expires in {h}h {m}m");
            }
        }
        None => println!("Logged in (unable to decode token details)"),
    }
    Ok(())
}

/// Decode JWT claims without verifying the signature.
/// The CLI only needs `github_user` (for display) and `exp` (for status).
fn decode_jwt_claims(jwt: &str) -> Option<(String, u64)> {
    let payload = jwt.split('.').nth(1)?;
    let padded = match payload.len() % 4 {
        2 => format!("{payload}=="),
        3 => format!("{payload}="),
        _ => payload.to_string(),
    };
    let bytes = base64_url_decode(&padded)?;
    let json: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    let github_user = json.get("github_user")?.as_str()?.to_string();
    let exp = json.get("exp")?.as_u64()?;
    Some((github_user, exp))
}

fn base64_url_decode(s: &str) -> Option<Vec<u8>> {
    let s = s.replace('-', "+").replace('_', "/");
    let mut out = Vec::new();
    let mut buf: u32 = 0;
    let mut bits: u32 = 0;
    for c in s.chars() {
        if c == '=' {
            break;
        }
        let val = match c {
            'A'..='Z' => c as u32 - 'A' as u32,
            'a'..='z' => c as u32 - 'a' as u32 + 26,
            '0'..='9' => c as u32 - '0' as u32 + 52,
            '+' => 62,
            '/' => 63,
            _ => return None,
        };
        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    Some(out)
}

fn read_line() -> Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
