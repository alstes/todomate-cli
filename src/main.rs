mod api;
mod auth;
mod cli;
mod commands;
mod config;
mod error;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command, ConfigCommand};

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
        Command::Auth { action } => {
            let cfg = config::load()?;
            auth::handle(action, &cfg.api_base_url)
        }
        Command::Config { action } => handle_config(action),
        other => {
            let cfg = config::load()?;
            let client = api::ApiClient::new(cfg.api_base_url);
            handle_api_command(&client, other, cli.json)
        }
    }
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
        Command::Reorder(args) => commands::todos::reorder(client, args, json),
        Command::Goal { action } => commands::goals::handle(client, action, json),
        Command::Vision { action } => commands::vision::handle(client, action, json),
        Command::Auth { .. } | Command::Config { .. } => unreachable!(),
    }
}
