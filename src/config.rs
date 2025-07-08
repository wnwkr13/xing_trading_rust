use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub app_key: String,
    pub app_secret: String,
    pub token_url: String,
    pub token_cache_file: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            app_key: env::var("XING_APP_KEY").expect("XING_APP_KEY 없음"),
            app_secret: env::var("XING_SECRET_KEY").expect("XING_APP_SECRET 없음"),
            token_url: env::var("XING_TOKEN_DOMAI").expect("XING_TOKEN_DOMAI 없음"),
            token_cache_file: env::var("KIS_TOKEN_CACHE_FILE")
                .unwrap_or("kis_token_cache.json".to_string()),
        }
    }
}
