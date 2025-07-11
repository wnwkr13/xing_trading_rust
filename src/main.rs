mod auth;
mod config;
mod constant;
mod http;
mod quotation;
mod types;
mod websocket;
mod zmq;

use crate::auth::oauth::get_access_token;
use crate::auth::ws_auth::get_ws_approval_key;
use crate::config::AppConfig;
use log::{error, info};

#[tokio::main]
async fn main() {
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let config = AppConfig::from_env();

    match get_access_token(&config).await {
        Ok(token) => info!("Access Token: {}", token),
        Err(e) => error!("토큰 발급 실패: {:?}", e),
    }

    match get_ws_approval_key(&config).await {
        Ok(key) => info!("WebSocket Approval Key: {}", key),
        Err(e) => error!("WS Approval Key 발급 실패: {:?}", e),
    }
}
