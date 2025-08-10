use sqlx::postgres::PgPoolOptions;
use tsukimi_core::models::Engine;

#[derive(Clone)]
pub struct DatabaseService {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DatabaseService {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn get_engines(&self) -> Result<Vec<Engine>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT *
            FROM engines
            ORDER BY name ASC
        "#,
        )
        .fetch_all(&self.pool)
        .await
    }
}
