use clap::{Parser, Subcommand};
use log::error;

use crate::services::credentials::{read_token, store_token};

pub mod commands;
pub mod error;
// pub mod config;
pub mod api;
pub mod services;

#[derive(Parser)]
#[command(author = "flender <tristan.deloeil@gmail.com>", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the Tsukimi CLI
    Init {
        engine: Option<String>,
    },

    Login,
    Logout,
    /// Show the current user.
    Whoami,

    Push,
    Pull,

    // API engines
    /// List installed engines.
    List,
    /// Search for an engine.
    Search {
        query: Option<String>,
    },
    /// Install a package.
    Install {
        package: String,
    },
    /// Uninstall a package.
    Uninstall {
        package: String,
    },
    /// Update a package.
    /// If no package is specified, update all packages
    Update {
        package: String,
    },
    /// Show the status of the CLI and its packages.
    Outdated,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let response = match cli.command {
        Commands::Login => commands::login::execute().await,
        Commands::Whoami => commands::whoami::execute().await,
        _ => todo!("Commands..."),
    };

    if let Err(e) = response {
        error!("Error: {}", e);
    }
}
