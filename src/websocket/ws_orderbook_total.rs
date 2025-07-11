use crate::types::orderbook::OrderbookMessage;
use crate::websocket::client::{ClientConfig, WebSocketClient};
use crate::websocket::handler::MessageHandler;
use crate::zmq::publisher::ZmqPublisher;
use futures_util::StreamExt;
use serde_json::Value;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message}; // for .next()

#[derive(Debug, Clone)]
pub struct OrderbookHandlerConfig {
    pub token: String,
    pub tr_cd: String, // <- 시장별로 "H1_", "HA_", "UH1" 등 지정
    pub tr_key: String,
    pub zmq_endpoint: String,
    pub print_console: bool,
    pub save_to_file: bool,
    pub file_path: Option<String>,
}

impl Default for OrderbookHandlerConfig {
    fn default() -> Self {
        Self {
            token: "".to_string(),
            tr_cd: "UH1".to_string(), // 기본값: KOSPI
            tr_key: "".to_string(),
            zmq_endpoint: "tcp://0.0.0.0:5557".to_string(),
            print_console: true,
            save_to_file: false,
            file_path: None,
        }
    }
}

pub struct OrderbookHandler {
    config: OrderbookHandlerConfig,
}

impl OrderbookHandler {
    pub fn new(config: OrderbookHandlerConfig) -> Self {
        Self { config }
    }
}

impl MessageHandler for OrderbookHandler {
    type Message = OrderbookMessage;

    fn subscription_message(&self) -> Value {
        serde_json::json!({
            "header": {
                "token": self.config.token,
                "tr_type": "3", // 실시간 시세 등록
            },
            "body": {
                "tr_cd": self.config.tr_cd,   // <- 시장별로 동적으로 지정
                "tr_key": self.config.tr_key,
            }
        })
    }
}

fn save_orderbook(orderbook: &OrderbookMessage, file_path: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    writeln!(file, "{:?}", orderbook)?;
    Ok(())
}

pub async fn run_orderbook_stream(
    client_config: ClientConfig,
    handler_config: OrderbookHandlerConfig,
) -> Result<(), Box<dyn Error>> {
    let handler = OrderbookHandler::new(handler_config.clone());
    let client = WebSocketClient::new(client_config, handler);

    let publisher = ZmqPublisher::bind(&handler_config.zmq_endpoint)?;

    client
        .run_with_callback(move |orderbook: &OrderbookMessage| {
            if handler_config.print_console {
                println!("{:?}", orderbook);
            }
            if handler_config.save_to_file {
                if let Some(ref path) = handler_config.file_path {
                    if let Err(e) = save_orderbook(orderbook, path) {
                        eprintln!("Orderbook 데이터 저장 실패: {}", e);
                    }
                }
            }
            let json = serde_json::to_string(orderbook).unwrap();
            if let Err(e) = publisher.send(json.as_str()) {
                eprintln!("ZeroMQ publish 실패: {}", e);
            }
        })
        .await?;

    Ok(())
}
