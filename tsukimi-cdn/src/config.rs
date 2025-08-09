use figment::{Figment, providers::Env};
use getset::{CopyGetters, Getters};
use serde::Deserialize;

#[derive(Deserialize, Getters, CopyGetters)]
pub struct Configuration {
    #[getset(get_copy = "pub")]
    port: u16,
}

pub fn get_configuration() -> Result<Configuration, figment::Error> {
    Figment::new()
        .merge(Env::raw().map(|key| match key.as_str().split_once('_') {
            Some((prefix, suffix)) => format!("{}.{}", prefix, suffix).into(),
            _ => key.into(),
        }))
        .extract()
}
