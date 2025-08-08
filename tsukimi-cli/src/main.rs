use clap::{Parser, Subcommand};

// pub mod commands;
// pub mod config;
pub mod api;
pub mod auth;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        _ => todo!("Commands..."),
    }
}
