use crate::api::ApiClient;
use crate::cli::VisionCommand;
use crate::output;
use anyhow::Result;

pub fn handle(client: &ApiClient, action: &VisionCommand, json: bool) -> Result<()> {
    match action {
        VisionCommand::Show => {
            let vision = client.get_vision()?;
            if json {
                output::print_json(&vision);
            } else {
                output::print_vision(&vision);
            }
        }
        VisionCommand::Set { text } => {
            let vision = client.update_vision(text)?;
            if json {
                output::print_json(&vision);
            } else {
                println!("Vision updated.");
            }
        }
    }
    Ok(())
}
