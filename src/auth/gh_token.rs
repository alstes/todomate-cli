use crate::error::CliError;
use anyhow::Result;

/// Run `gh auth token` and return the GitHub access token.
pub fn get_token() -> Result<String> {
    let output = std::process::Command::new("gh")
        .args(["auth", "token"])
        .output()
        .map_err(|_| {
            CliError::AuthFailed(
                "`gh` not found. Install GitHub CLI (https://cli.github.com) or use `todo auth login --device-flow`.".to_string(),
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CliError::AuthFailed(format!(
            "`gh auth token` failed: {}. Run `gh auth login` first.",
            stderr.trim()
        ))
        .into());
    }

    let token = String::from_utf8(output.stdout)
        .map_err(|_| CliError::AuthFailed("Invalid UTF-8 from `gh auth token`".to_string()))?
        .trim()
        .to_string();

    if token.is_empty() {
        return Err(CliError::AuthFailed(
            "`gh auth token` returned an empty token. Run `gh auth login` first.".to_string(),
        )
        .into());
    }

    Ok(token)
}
