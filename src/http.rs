use reqwest::{Client, Method, Response};
use serde_json::Value;

/// 공통 HTTP API 호출 함수 (POST/GET, 헤더/바디/쿼리 지원)
pub async fn execute_api_call(
    client: &Client,
    url: &str,
    method: Method,
    headers: Option<reqwest::header::HeaderMap>,
    body: Option<Value>,
    query: Option<&[(&str, &str)]>,
) -> Result<Response, reqwest::Error> {
    let mut req = client.request(method, url);
    if let Some(h) = headers {
        req = req.headers(h);
    }
    if let Some(q) = query {
        req = req.query(q);
    }
    if let Some(b) = body {
        req = req.json(&b);
    }
    let resp = req.send().await?;
    Ok(resp)
}
