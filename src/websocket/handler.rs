// WebSocket 메시지 처리를 위한 공통 트레이트와 타입을 정의합니다.
// 이 코드에서는 ws 메시지 parsing과 handler trait(message handler) 정의되어있음음

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

/// 파싱된 WebSocket 메시지의 종류
#[derive(Debug)]
pub enum ParsedMessage<T> {
    /// 성공적으로 파싱된 데이터 메시지
    Message(T),
    /// Pong 메시지
    Pong,
    /// Ping 메시지
    Ping,
    /// 연결 종료 메시지
    Closed,
    /// 알 수 없는 메시지
    Unknown,
}

/// WebSocket 메시지를 처리하는 핸들러의 동작을 정의하는 트레이트
///
/// 이 트레이트를 구현하여 특정 스트림(Ticker, Trade 등)에 대한
/// 구독 메시지 생성 및 데이터 파싱 로직을 정의할 수 있습니다.
pub trait MessageHandler: Send + Sync {
    /// 핸들러가 처리할 메시지의 타입
    type Message: DeserializeOwned + Debug + Send + Serialize;

    /// WebSocket 구독을 위한 메시지를 생성합니다.
    fn subscription_message(&self) -> Value;

    /// 원시 WebSocket 메시지(TungsteniteMessage)를 파싱하여
    /// 애플리케이션에서 사용할 수 있는 `ParsedMessage`로 변환합니다.
    fn parse_raw_message(&self, msg: &TungsteniteMessage) -> ParsedMessage<Self::Message> {
        match msg {
            TungsteniteMessage::Binary(bin) => {
                // 데이터 메시지로 파싱 시도
                if let Ok(json) = serde_json::from_slice::<Self::Message>(bin) {
                    return ParsedMessage::Message(json);
                }
                // 에러 구조가 없으므로, 그냥 사람이 볼 수 있게 출력
                match String::from_utf8(bin.to_vec()) {
                    Ok(decoded) => {
                        eprintln!("수신된 바이너리(텍스트): {}", decoded);
                    }
                    Err(e) => {
                        eprintln!("UTF-8 디코딩 실패: {}", e);
                    }
                }
                ParsedMessage::Unknown
            }
            TungsteniteMessage::Text(text) => {
                // 그냥 로그로 남기고 Unknown 처리
                eprintln!("수신된 텍스트 메시지: {}", text);
                ParsedMessage::Unknown
            }
            TungsteniteMessage::Ping(_) => ParsedMessage::Ping,
            TungsteniteMessage::Pong(_) => ParsedMessage::Pong,
            TungsteniteMessage::Close(_) => ParsedMessage::Closed,
            _ => ParsedMessage::Unknown,
        }
    }
}
