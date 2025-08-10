use serde::Serialize;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Engine {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub current_version: String,
    // pub created_at: String,
    // pub updated_at: String,
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
