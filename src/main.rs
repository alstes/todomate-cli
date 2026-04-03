mod api;
mod auth;
mod cli;
mod commands;
mod config;
mod error;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{AuthCommand, Cli, Command, ConfigCommand};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    output::init(cli.no_color);

    match &cli.command {
        Command::Auth { action } => handle_auth(action),
        Command::Config { action } => handle_config(action),
        other => {
            let cfg = config::load()?;
            let client = api::ApiClient::new(cfg.api_base_url);
            handle_api_command(&client, other, cli.json)
        }
    }
}

fn handle_auth(action: &AuthCommand) -> Result<()> {
    match action {
        AuthCommand::Login { device_flow } => {
            if *device_flow {
                eprintln!("Device Flow is not yet implemented. See Phase 8.");
                std::process::exit(1);
            }

            // Get GitHub token via `gh auth token`
            let github_token = auth::gh_token::get_token()?;

            // Prompt for API key
            eprint!("Enter your API key (from todo.ac → Settings → API Access): ");
            let subscription_key = read_line_stdin()?;
            if subscription_key.is_empty() {
                eprintln!("error: API key cannot be empty.");
                std::process::exit(1);
            }

            // Exchange for JWT
            let cfg = config::load()?;
            let client = api::ApiClient::new(cfg.api_base_url);
            let auth_resp = client.exchange_github_token(&github_token, &subscription_key)?;

            // Store all three in keychain / file
            auth::token_store::save(&auth::token_store::Credentials {
                jwt: auth_resp.access_token,
                refresh_token: auth_resp.refresh_token,
                subscription_key,
            })?;

            println!("Logged in as @{}", auth_resp.user.login);
            Ok(())
        }
        AuthCommand::Logout => {
            auth::token_store::delete()?;
            println!("Logged out.");
            Ok(())
        }
        AuthCommand::Status => {
            let creds = auth::token_store::load()?;
            let claims = decode_jwt_claims(&creds.jwt);
            match claims {
                Some((github_user, exp)) => {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    if exp <= now {
                        println!(
                            "Logged in as @{github_user} (token expired — run `todo auth login`)"
                        );
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
    }
}

/// Minimal JWT claims decoder — does not verify signature (CLI doesn't need the secret).
fn decode_jwt_claims(jwt: &str) -> Option<(String, u64)> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    let payload = parts[1];
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

fn read_line_stdin() -> Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn handle_config(action: &ConfigCommand) -> Result<()> {
    match action {
        ConfigCommand::Set { key, value } => {
            config::set_value(key, value)?;
            println!("Set {key} = {value}");
        }
        ConfigCommand::Get { key } => {
            let value = config::get_value(key)?;
            println!("{value}");
        }
        ConfigCommand::Reset => {
            config::reset()?;
            println!("Config reset to defaults.");
        }
    }
    Ok(())
}

fn handle_api_command(client: &api::ApiClient, command: &Command, json: bool) -> Result<()> {
    match command {
        Command::List(args) => commands::todos::list(client, args, json),
        Command::Add(args) => commands::todos::add(client, args, json),
        Command::Done { id } => commands::todos::done(client, id, json),
        Command::Edit(args) => commands::todos::edit(client, args, json),
        Command::Rm(args) => commands::todos::rm(client, args),
        Command::Goal { action } => commands::goals::handle(client, action, json),
        Command::Vision { action } => commands::vision::handle(client, action, json),
        Command::Auth { .. } | Command::Config { .. } => unreachable!(),
    }
}
