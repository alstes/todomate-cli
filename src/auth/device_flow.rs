// GitHub Device Flow — used when `gh` is unavailable.
// Wired up in Phase 8 (optional). Placeholder for now.

use anyhow::Result;

#[allow(dead_code)]
pub fn authenticate(_client_id: &str) -> Result<String> {
    unimplemented!(
        "Device Flow is not yet implemented. Use `todo auth login` (requires gh CLI) for now."
    )
}
