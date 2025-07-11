// WebSocket 연결, 재연결, 메시지 수신 등 공통 로직을 처리하는 범용 클라이언트입니다.
// src/websocket/client.rs

use super::handler::{MessageHandler, ParsedMessage};
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::Message as TungsteniteMessage};

/// WebSocket 클라이언트의 설정(연결, 재연결, 메시지수신) 등 공통 로직 담당 범용 구조체
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub url: String,
    pub reconnect_interval: Duration,
    pub max_reconnect_attempts: usize,
    pub ping_interval: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            url: "wss://openapi.ls-sec.co.kr:9443/websocket".to_string(),
            reconnect_interval: Duration::from_secs(5),
            max_reconnect_attempts: 10,
            ping_interval: Duration::from_secs(60),
        }
    }
}

/// 제네릭 `MessageHandler`를 사용하여 WebSocket 통신을 관리하는 클라이언트
pub struct WebSocketClient<H: MessageHandler> {
    config: ClientConfig,
    handler: H,
}

impl<H: MessageHandler + 'static> WebSocketClient<H> {
    pub fn new(config: ClientConfig, handler: H) -> Self {
        Self { config, handler }
    }

    /// 콜백 함수 기반 WebSocket 메시지 처리
    /// 외부에서 FnMut(&H::Message)를 넘기면, 메시지 수신 시마다 콜백을 실행합니다.
    pub async fn run_with_callback<F>(&self, mut on_msg: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&H::Message) + Send + 'static,
        H::Message: serde::Serialize,
    {
        let mut reconnect_count = 0;
        loop {
            match self.connect_and_listen_with_callback(&mut on_msg).await {
                Ok(_) => {
                    println!("WebSocket 연결이 정상적으로 종료되었습니다. 재연결을 시도합니다.");
                    reconnect_count = 0;
                }
                Err(e) => {
                    eprintln!("WebSocket 에러 발생: {}. 재연결을 시도합니다.", e);
                    reconnect_count += 1;
                    if reconnect_count >= self.config.max_reconnect_attempts {
                        eprintln!("최대 재연결 시도 횟수를 초과했습니다.");
                        return Err(e);
                    }
                }
            }
            println!(
                "{}초 후 재연결... ({}/{})",
                self.config.reconnect_interval.as_secs(),
                reconnect_count,
                self.config.max_reconnect_attempts
            );
            sleep(self.config.reconnect_interval).await;
        }
    }

    /// 콜백 기반 실제 WebSocket 연결 및 메시지 수신/발신 로직
    async fn connect_and_listen_with_callback<F>(
        &self,
        on_msg: &mut F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&H::Message) + Send + 'static,
        H::Message: serde::Serialize,
    {
        println!("WebSocket에 연결 중... URL: {}", self.config.url);
        let (ws_stream, _) = connect_async(&self.config.url).await?;
        let (mut write, mut read) = ws_stream.split();
        println!("✅ WebSocket 연결 성공!");

        // 핸들러로부터 구독 메시지를 받아 전송
        let subscribe_msg = self.handler.subscription_message();
        write
            .send(TungsteniteMessage::Text(subscribe_msg.to_string().into()))
            .await?;
        println!("📡 구독 메시지 전송 완료: {}", subscribe_msg);

        let mut ping_interval = tokio::time::interval(self.config.ping_interval);

        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    if let Err(e) = write.send(TungsteniteMessage::Ping(Vec::new())).await {
                        eprintln!("Ping 전송 실패: {}", e);
                        break;
                    }
                }
                msg = read.next() => {
                    match msg {
                        Some(Ok(TungsteniteMessage::Binary(binary))) => {
                            match self.handler.parse_raw_message(&TungsteniteMessage::Binary(binary)) {
                                ParsedMessage::Message(m) => {
                                    on_msg(&m); // 콜백 실행
                                }
                                ParsedMessage::Pong => {}
                                ParsedMessage::Ping => {
                                    write.send(TungsteniteMessage::Pong(Vec::new())).await?;
                                }
                                ParsedMessage::Closed => {
                                    println!("서버로부터 연결 종료 메시지를 받았습니다.");
                                    break;
                                }
                                ParsedMessage::Unknown => {
                                    eprintln!("알 수 없는 바이너리 메시지 수신.");
                                }
                            }
                        }
                        Some(Ok(TungsteniteMessage::Ping(payload))) => {
                            write.send(TungsteniteMessage::Pong(payload)).await?;
                        }
                        Some(Ok(TungsteniteMessage::Pong(_))) => {}
                        Some(Ok(TungsteniteMessage::Text(txt))) => {
                            eprintln!("수신된 텍스트 메시지: {}", txt);
                        }
                        Some(Ok(_)) => {}
                        Some(Err(e)) => {
                            eprintln!("WebSocket 메시지 수신 에러: {}", e);
                            return Err(e.into());
                        }
                        None => {
                            println!("WebSocket 스트림이 종료되었습니다.");
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
