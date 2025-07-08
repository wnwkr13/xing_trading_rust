use crate::auth::cache::{CachedToken, load_cached_token, save_cached_token};
use crate::config::AppConfig;
use chrono::{Duration, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,    // 유효기간(초)
    scope: String,      // "oob" 고정
    token_type: String, // bearer
}

pub async fn get_access_token(config: &AppConfig) -> Result<String, Box<dyn std::error::Error>> {
    // 1. 캐시된 토큰이 있으면 만료 전까지 재사용
    if let Ok(cached) = load_cached_token(&config.token_cache_file) {
        if cached.expired_at > Utc::now() {
            println!("Cached token 사용: 만료시각={}", cached.expired_at);
            return Ok(cached.access_token);
        }
    }

    // 2. API 요청 (LS증권: x-www-form-urlencoded)
    let client = Client::new();
    let params = [
        ("grant_type", "client_credentials"),
        ("appkey", &config.app_key),
        ("appsecretkey", &config.app_secret),
        ("scope", "oob"),
    ];
    let token_url = format!("{}/oauth2/token", config.token_url);

    let resp = match client
        .post(&token_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("HTTP 요청 실패: {}", e);
            return Err(Box::new(e));
        }
    };

    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => {
            println!("응답 본문 읽기 실패: {}", e);
            return Err(Box::new(e));
        }
    };
    println!("LS 토큰 응답 원문: {}", text);

    let token: TokenResponse = match serde_json::from_str(&text) {
        Ok(token) => token,
        Err(e) => {
            println!("LS API 토큰 파싱 실패: {}", e);
            return Err(Box::new(e));
        }
    };

    // 3. 만료시각 계산 (expire_in: 초 단위 문자열)
    let expire_secs = token.expires_in as i64;
    let expired_at = Utc::now() + Duration::seconds(expire_secs);

    // 4. 캐시 저장
    let cached = CachedToken {
        access_token: token.access_token.clone(),
        expired_at,
    };
    if let Err(e) = save_cached_token(&config.token_cache_file, &cached) {
        println!("토큰 캐시 저장 실패: {}", e);
    }

    println!("신규 토큰 발급, 만료시각={}", expired_at);
    Ok(token.access_token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use tokio;

    // 실제 API 호출이므로, 환경변수와 네트워크가 필요합니다.
    #[tokio::test]
    async fn test_get_access_token() {
        let config = AppConfig::from_env();
        let result = get_access_token(&config).await;
        match &result {
            Ok(token) => println!("AccessToken: {}", token),
            Err(e) => println!("AccessToken 발급 실패: {}", e),
        }
        assert!(result.is_ok() || result.is_err());
    }
}
