use tabled::{Table, Tabled, settings::Style};

use crate::{error::CliResult, services::api::ApiService};

#[derive(Tabled)]
struct ListItem {
    name: String,
    version: String,
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

pub async fn execute() -> CliResult {
    let list = ApiService::default().fetch_engines(None).await?;

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
