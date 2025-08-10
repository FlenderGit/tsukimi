use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tsukimi_core::models::Engine;

#[derive(Clone)]
pub struct DatabaseService {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiPagination {
    #[serde(default = "default_query")]
    pub query: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_query() -> String {
    String::new()
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    10
}

impl DatabaseService {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn get_engines(&self, pagination: ApiPagination) -> Result<Vec<Engine>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT *
            FROM engines
            WHERE name ILIKE $1
            ORDER BY name ASC
            LIMIT $2 OFFSET $3
        "#,
        )
        .bind(format!("%{}%", pagination.query))
        .bind(pagination.per_page as i64)
        .bind((pagination.page as i64 - 1) * pagination.per_page as i64)
        .fetch_all(&self.pool)
        .await
    }
}
