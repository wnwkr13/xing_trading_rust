use crate::config::AppConfig;

use log::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct ApprovalKeyResponse {
    pub approval_key: String,
}

pub async fn get_ws_approval_key(config: &AppConfig) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let body = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": &config.app_key,
        "secretkey": &config.app_secret,
    });

    let approval_url = format!("{}/oauth2/token", config.token_url);

    let resp = client
        .post(&approval_url)
        .header("Content-Type", "application/json; charset=UTF-8")
        .json(&body)
        .send()
        .await?;

    let text = resp.text().await?;
    debug!("KIS ApprovalKey 응답 원문: {}", text);

    let key_result: Result<ApprovalKeyResponse, _> = serde_json::from_str(&text);
    let key = match key_result {
        Ok(key) => key,
        Err(_) => {
            error!("KIS ApprovalKey 발급 실패 응답: {}", text);
            return Err(format!("KIS ApprovalKey 발급 실패: {}", text).into());
        }
    };

    info!("신규 ApprovalKey 발급: {}", key.approval_key);
    Ok(key.approval_key)
}

fn can_request_token(last_request_file: &str, min_interval_secs: u64) -> bool {
    if let Ok(data) = fs::read_to_string(last_request_file) {
        if let Ok(last) = data.parse::<u64>() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return now - last >= min_interval_secs;
        }
    }
    true // 파일 없으면 요청 허용
}

fn update_last_request_time(last_request_file: &str) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let _ = fs::write(last_request_file, now.to_string());
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::config::AppConfig;

//     #[tokio::test]
//     async fn test_get_ws_approval_key() {
//         let config = AppConfig::from_env();
//         let result = get_ws_approval_key(&config).await;
//         println!("ApprovalKey result: {:?}", result);
//         assert!(result.is_ok());
//     }
// }
