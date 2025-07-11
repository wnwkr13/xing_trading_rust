use serde::Deserialize;
use std::fs;

use xing_trading_rust::websocket::client::ClientConfig;
use xing_trading_rust::websocket::ws_orderbook_total::{
    OrderbookHandlerConfig, run_orderbook_stream,
};

#[derive(Deserialize)]
struct TokenCache {
    access_token: String,
    expired_at: String,
}

fn load_token_from_file(path: &str) -> String {
    let data = fs::read_to_string(path).expect("토큰 파일 읽기 실패");
    let cache: TokenCache = serde_json::from_str(&data).expect("JSON 파싱 실패");
    cache.access_token
}

#[tokio::test]
async fn test_real_orderbook_stream() {
    let token = load_token_from_file("kis_token_cache.json");
    let client_config = ClientConfig {
        url: "wss://openapi.ls-sec.co.kr:9443/websocket".to_string(),
        reconnect_interval: std::time::Duration::from_secs(5),
        max_reconnect_attempts: 10,
        ping_interval: std::time::Duration::from_secs(60),
    };
    let handler_config = OrderbookHandlerConfig {
        token,
        tr_cd: "UH1".to_string(),         // 실제 사용 값
        tr_key: "U005930   ".to_string(), // 실제 사용 값 (공백 포함)
        print_console: true,
        save_to_file: false,
        file_path: None,
        zmq_endpoint: "tcp://127.0.0.1:5557".to_string(),
    };

    // 10초 동안만 실행
    let timeout = std::time::Duration::from_secs(10);
    let result =
        tokio::time::timeout(timeout, run_orderbook_stream(client_config, handler_config)).await;
    assert!(result.is_ok(), "Timeout or error occurred");
}
