use crate::{
    commands::{
        init::InitCommandParams, install::InstallCommandParams, search::SearchCommandParams,
    },
    error::CliResult,
};
use clap::{Parser, Subcommand};

pub mod api;
pub mod commands;
pub mod error;
pub mod models;
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

    // User commands
    /// Login to Github using Device Authorization Flow or Oauth2.
    /// Used to push a patch to the API.
    Login,
    /// Logout from the current session.
    Logout,
    /// Show the current user.
    Whoami,

    // API engines commands
    /// List installed engines.
    List,
    /// Search for an engine.
    Search(SearchCommandParams),
    /// Install a package.
    Install(InstallCommandParams),
    /// Uninstall a package.
    Uninstall { package: String },
    /// Update a package.
    /// If no package is specified, update all packages
    Update { package: String },
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
        Commands::Logout => commands::logout::execute().await,
        Commands::Init(params) => commands::init::execute(params).await,
        Commands::Install(params) => commands::install::execute(params).await,
        Commands::Search(params) => commands::search::execute(params).await,
        Commands::List => commands::list::execute().await,
        // Commands::List => test(),
        _ => todo!("Commands..."),
    };

    if let Err(e) = response {
        eprintln!("An error occurred: {}", e);
    }
}

use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
struct WasmState {
    table: ResourceTable,
    ctx: WasiCtx,
}
impl IoView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for WasmState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

fn test() -> CliResult {
    use wasmtime::{
        Config, Engine, Store,
        component::{Component, HasSelf, Linker, bindgen},
    };

    bindgen!("extension" in "../tsukimi-extension/wit/extension.wit");
    impl utils::Host for WasmState {
        fn read_file(
            &mut self,
            path: wasmtime::component::__internal::String,
        ) -> Option<wasmtime::component::__internal::String> {
            println!("Reading file at path: {}", path);
            Some("File content".to_string())
        }
    }

    let engine =
        Engine::new(Config::new().wasm_component_model(true)).expect("Failed to create engine");
    let mut store = Store::new(
        &engine,
        WasmState {
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_stdio().build(),
        },
    );
    let mut linker = Linker::new(&engine);

    let component = Component::from_file(
        &engine,
        "../target/wasm32-wasip2/release/extension_exemple.wasm",
    )
    .expect("Failed to load component");

    wasmtime_wasi::p2::add_to_linker_sync(&mut linker).expect("Failed to add WASI to linker");
    Extension::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)
        .expect("Failed to add extension to linker");

    let ex = Extension::instantiate(&mut store, &component, &linker)
        .expect("Failed to instantiate component");
    let result = ex.iadd().call_add_to_gitignore(store);
    println!("Result: {:?}", result);

    Ok(())
}
