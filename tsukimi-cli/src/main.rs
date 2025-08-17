use clap::{Parser, Subcommand};
use inquire::{Confirm, InquireError, Select};
use log::error;
use tabled::{Table, settings::Style};

use crate::{
    api::ApiError,
    commands::init::InitCommandParams,
    services::{api::ApiService, credentials::delete_token},
};

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
    Init(InitCommandParams),

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
        Commands::Logout => {
            delete_token();
            todo!()
        }
        Commands::Init(params) => commands::init::execute(params).await,
        Commands::List => commands::list::execute().await,
        _ => todo!("Commands..."),
    };

    if let Err(e) = response {
        eprintln!("An error occurred: {}", e);
    }
}
