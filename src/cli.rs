use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "todo",
    version,
    about = "todo.ac CLI — manage your todos from the terminal"
)]
pub struct Cli {
    /// Output raw JSON instead of formatted text
    #[arg(long, global = true)]
    pub json: bool,

    /// Disable color output
    #[arg(long, global = true, env = "NO_COLOR")]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        action: AuthCommand,
    },
    /// List todos
    #[command(alias = "ls")]
    List(ListArgs),
    /// Create a new todo
    #[command(alias = "a")]
    Add(AddArgs),
    /// Mark a todo as complete
    Done { id: String },
    /// Update a todo's fields
    Edit(EditArgs),
    /// Delete a todo
    #[command(alias = "delete")]
    Rm(RmArgs),
    /// Goal management subcommands
    Goal {
        #[command(subcommand)]
        action: GoalCommand,
    },
    /// Vision management subcommands
    Vision {
        #[command(subcommand)]
        action: VisionCommand,
    },
    /// CLI configuration
    Config {
        #[command(subcommand)]
        action: ConfigCommand,
    },
}

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Log in via GitHub. Tries gh auth token first; use --device-flow for standalone flow.
    Login {
        /// Use GitHub Device Flow instead of gh auth token
        #[arg(long)]
        device_flow: bool,
    },
    /// Delete stored credentials from keychain
    Logout,
    /// Print current auth state
    Status,
}

#[derive(Args)]
pub struct ListArgs {
    /// Show completed todos (default: active only)
    #[arg(short = 'c', long)]
    pub completed: bool,

    /// Show all todos regardless of completion
    #[arg(short = 'a', long)]
    pub all: bool,

    /// Filter by priority: low | medium | high
    #[arg(short = 'p', long, value_name = "LEVEL")]
    pub priority: Option<String>,

    /// Maximum number of todos to return (default: 50)
    #[arg(long, value_name = "N")]
    pub limit: Option<u32>,

    /// Number of todos to skip (for pagination)
    #[arg(long, value_name = "N")]
    pub offset: Option<u32>,
}

#[derive(Args)]
pub struct AddArgs {
    /// Todo text
    pub text: String,

    /// Priority: low | medium | high (default: medium)
    #[arg(short = 'p', long, value_name = "LEVEL")]
    pub priority: Option<String>,

    /// Long description
    #[arg(short = 'd', long, value_name = "TEXT")]
    pub description: Option<String>,

    /// Notes
    #[arg(short = 'n', long, value_name = "TEXT")]
    pub notes: Option<String>,

    /// Associate with a goal ID (repeatable)
    #[arg(long, value_name = "ID")]
    pub goal: Vec<String>,
}

#[derive(Args)]
pub struct EditArgs {
    /// Todo ID
    pub id: String,

    #[arg(long, value_name = "TEXT")]
    pub text: Option<String>,

    #[arg(long, value_name = "TEXT")]
    pub description: Option<String>,

    #[arg(long, value_name = "TEXT")]
    pub notes: Option<String>,

    #[arg(long, value_name = "LEVEL")]
    pub priority: Option<String>,

    /// Mark as not completed
    #[arg(long)]
    pub uncomplete: bool,
}

#[derive(Args)]
pub struct RmArgs {
    /// Todo ID
    pub id: String,

    /// Skip confirmation prompt
    #[arg(long)]
    pub force: bool,
}

#[derive(Subcommand)]
pub enum GoalCommand {
    /// List goals
    List {
        #[arg(short = 'c', long)]
        completed: bool,
        #[arg(short = 'a', long)]
        all: bool,
    },
    /// Create a new goal
    Add {
        text: String,
        #[arg(long, value_name = "TEXT")]
        notes: Option<String>,
    },
    /// Update a goal's fields
    Edit {
        id: String,
        #[arg(long, value_name = "TEXT")]
        text: Option<String>,
        #[arg(long, value_name = "TEXT")]
        notes: Option<String>,
        #[arg(long)]
        done: bool,
        #[arg(long)]
        uncomplete: bool,
    },
    /// Mark a goal as complete
    Done { id: String },
    /// Delete a goal
    Rm {
        id: String,
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum VisionCommand {
    /// Show your vision
    Show,
    /// Set your vision
    Set { text: String },
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Set a config value
    Set { key: String, value: String },
    /// Get a config value
    Get { key: String },
    /// Reset all config to defaults
    Reset,
}
