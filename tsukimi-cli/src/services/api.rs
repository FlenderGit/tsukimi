use crate::api::ApiError;

// #[if]
static BASE_URL: &str = "http://localhost:3000";

pub struct ApiService {
    base_url: String,
}

impl Default for ApiService {
    fn default() -> Self {
        Self {
            base_url: BASE_URL.to_string(),
        }
    }
}

impl ApiService {
    pub fn new(base_url: String) -> Self {
        ApiService { base_url }
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        )
    }

    fn get_client(&self) -> reqwest::Client {
        reqwest::Client::new()
    }

    pub async fn fetch_engine(
        &self,
        engine_id: &str,
    ) -> Result<tsukimi_core::models::Engine, ApiError> {
        let url = self.build_url(&format!("engines/{}", engine_id));
        let client = self.get_client();

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let engine: tsukimi_core::models::Engine = response.json().await?;
            Ok(engine)
        } else {
            Err(ApiError::NetworkError(
                response.text().await.unwrap_or_default(),
            ))
        }
    }

    pub async fn fetch_engines(
        &self,
        query: Option<String>,
    ) -> Result<Vec<tsukimi_core::models::Engine>, ApiError> {
        let url = self.build_url("engines");
        let client = self.get_client();

        let response = client.get(&url).query(&[("query", query)]).send().await?;

        if response.status().is_success() {
            let engines: Vec<tsukimi_core::models::Engine> = response.json().await?;
            Ok(engines)
        } else {
            Err(ApiError::NetworkError(
                response.text().await.unwrap_or_default(),
            ))
        }
    }
}
