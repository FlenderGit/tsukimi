use figment::{Figment, providers::Env};
use getset::{CopyGetters, Getters};
use serde::Deserialize;

#[derive(Deserialize, Getters, CopyGetters)]
pub struct Configuration {
    #[getset(get_copy = "pub")]
    port: u16,
    #[getset(get = "pub")]
    database: DatabaseConfiguration,
    #[getset(get = "pub")]
    env: Environment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
    Testing,
    Staging,
}
impl Environment {
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }
}
impl<'de> Deserialize<'de> for Environment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "testing" | "test" => Ok(Environment::Testing),
            "staging" => Ok(Environment::Staging),
            "production" | "prod" => Ok(Environment::Production),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown environment: {}",
                s
            ))),
        }
    }
}
#[derive(Deserialize, Getters)]
pub struct DatabaseConfiguration {
    name: String,
    password: String,
    username: String,
}

impl DatabaseConfiguration {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@postgres/{}?currentSchema=my_schema",
            self.username, self.password, self.name
        )
    }
}

pub fn get_configuration() -> Result<Configuration, figment::Error> {
    Figment::new()
        .merge(Env::raw().map(|key| match key.as_str().split_once('_') {
            Some((prefix, suffix)) => format!("{}.{}", prefix, suffix).into(),
            _ => key.into(),
        }))
        .extract()
}
