use tabled::{Table, Tabled, settings::Style};
use tsukimi_core::models::Version;

use crate::{error::CliResult, services::api::ApiService};

#[derive(clap::Args)]
pub struct SearchCommandParams {
    query: Option<String>,
}

#[derive(Tabled)]
struct ListItem {
    name: String,
    version: Version,
    description: String,
}

impl From<tsukimi_core::models::Engine> for ListItem {
    fn from(engine: tsukimi_core::models::Engine) -> Self {
        Self {
            name: engine.name,
            version: engine.current_version,
            description: engine.description,
        }
    }
}

pub async fn execute(params: SearchCommandParams) -> CliResult {
    let list = ApiService::default().fetch_engines(params.query).await?;

    match list.is_empty() {
        true => println!("No engines found."),
        false => {
            println!("Available engines:");
            let mut table = Table::new(
                list.into_iter()
                    .map(ListItem::from)
                    .collect::<Vec<ListItem>>(),
            );
            table.with(Style::rounded());
            println!("{table}");
        }
    }

    Ok(())
}
