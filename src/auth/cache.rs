// token caching/ expiration management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedToken {
    pub access_token: String,
    pub expired_at: DateTime<Utc>,
}

pub fn load_cached_token(path: &str) -> Result<CachedToken, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err("토큰 캐시 파일 없음".into());
    }
    let data = fs::read_to_string(path)?;
    let token: CachedToken = serde_json::from_str(&data)?;
    Ok(token)
}

pub fn save_cached_token(
    path: &str,
    token: &CachedToken,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string(token)?;
    fs::write(path, data)?;
    Ok(())
}
