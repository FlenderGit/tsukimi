use clap::{Parser, Subcommand};
use inquire::{Confirm, InquireError, Select};
use log::error;
use tabled::{Table, settings::Style};

use crate::{
    api::ApiError, commands::init::InitCommandParams, services::credentials::delete_token,
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

        Commands::List => {
            let list = vec![
                "tsukimi-cli",
                "tsukimi-api",
                "tsukimi-engine",
                "tsukimi-cdn",
            ];

            let mut table = Table::new(list);
            table.with(Style::rounded());
            println!("{table}");

            let options: Vec<&str> = vec![
                "Banana",
                "Apple",
                "Strawberry",
                "Grapes",
                "Lemon",
                "Tangerine",
                "Watermelon",
                "Orange",
                "Pear",
                "Avocado",
                "Pineapple",
            ];

            let ans: Result<&str, InquireError> =
                Select::new("What's your favorite fruit?", options).prompt();

            match ans {
                Ok(choice) => println!("{}! That's mine too!", choice),
                Err(_) => println!("There was an error, please try again"),
            }

            let ans = Confirm::new("Do you live in Brazil?")
                .with_default(false)
                .with_help_message("This data is stored for good reasons")
                .prompt();

            match ans {
                Ok(true) => println!("That's awesome!"),
                Ok(false) => println!("That's too bad, I've heard great things about it."),
                Err(_) => println!("Error with questionnaire, try again later"),
            }

            Ok(())
        }
        _ => todo!("Commands..."),
    };

    if let Err(e) = response {
        eprintln!("An error occurred: {}", e);
    }
}
