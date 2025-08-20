use std::path::Display;

use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres, Type};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Engine {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub current_version: Version,
    // pub created_at: String,
    // pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Type<Postgres> for Version {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("version")
    }
}

impl Decode<'_, Postgres> for Version {
    fn decode(
        value: <Postgres as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s: String = <String as Decode<Postgres>>::decode(value)?;
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(sqlx::error::BoxDynError::from(sqlx::error::Error::Decode(
                "Invalid version format".into(),
            )));
        }
        let major = parts[0].parse().map_err(|_| {
            sqlx::error::BoxDynError::from(sqlx::error::Error::Decode(
                "Failed to parse major version".into(),
            ))
        })?;
        let minor = parts[1].parse().map_err(|_| {
            sqlx::error::BoxDynError::from(sqlx::error::Error::Decode(
                "Failed to parse minor version".into(),
            ))
        })?;
        let patch = parts[2].parse().map_err(|_| {
            sqlx::error::BoxDynError::from(sqlx::error::Error::Decode(
                "Failed to parse patch version".into(),
            ))
        })?;
        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let version_str = format!("{}.{}.{}", self.major, self.minor, self.patch);
        serializer.serialize_str(&version_str)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version_str = String::deserialize(deserializer)?;
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return Err(serde::de::Error::custom("Invalid version format"));
        }
        let major = parts[0].parse().map_err(serde::de::Error::custom)?;
        let minor = parts[1].parse().map_err(serde::de::Error::custom)?;
        let patch = parts[2].parse().map_err(serde::de::Error::custom)?;
        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

pub struct EngineVersion {
    pub id: String,
    pub engine_id: String,
    pub version: String,
    pub description: String,
    pub created_at: String,
}

pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
