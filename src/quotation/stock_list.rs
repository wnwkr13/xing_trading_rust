use crate::config::AppConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct StockItem {
    pub hname: String,   // 종목명
    pub shcode: String,  // 단축코드
    pub expcode: String, // 확장코드
    pub etfchk: String,  // ETF구분
    pub nxt_chk: String, // NXT상장구분
    pub filler: String,  // filler
}

#[derive(Debug, Deserialize)]
struct StockListResponse {
    t9945OutBlock: Vec<StockItem>,
}

/// HFT 환경을 고려하여, etfchk 필터링은 iterator로 처리하고,
/// 필요시 HashMap<String, StockItem> 등으로 변환해 빠른 조회가 가능하도록 설계할 것.
pub async fn fetch_stock_list(
    config: &AppConfig,
    access_token: &str,
    tr_cd: &str,
    gubun: &str,              // "1" (KSP) or "2" (KSD)
    etfchk: Option<&str>,     // "0" or "1" or None(전체)
    etn_filter: Option<bool>, // Some(true): ETN만, Some(false): ETN 제외, None: 전체
    debug_print: bool,        // true면 상세 로그 출력
) -> Result<HashMap<String, StockItem>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("{}/stock/market-data", config.token_url);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json; charset=utf-8".parse()?);
    let bearer = format!("Bearer {}", access_token);
    headers.insert("authorization", bearer.parse()?);
    headers.insert("tr_cd", tr_cd.parse()?);
    headers.insert("tr_cont", "Y".parse()?);
    headers.insert("tr_cont_key", "".parse()?);

    let body = serde_json::json!({
        "t9945InBlock": {
            "gubun": gubun
        }
    });

    let resp = client
        .post(&url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;
    let text = resp.text().await?;
    if debug_print {
        println!("[DEBUG] API raw response: {}", text);
    }

    let parsed: serde_json::Value = match serde_json::from_str(&text) {
        Ok(val) => val,
        Err(e) => {
            if debug_print {
                println!("[DEBUG] JSON 파싱 실패: {}", e);
            }
            return Err(Box::new(e));
        }
    };
    if debug_print {
        println!("[DEBUG] 파싱된 JSON: {:?}", parsed);
    }

    // API 에러 코드/메시지 핸들링
    if let Some(rsp_cd) = parsed.get("rsp_cd") {
        if rsp_cd != "00000" {
            let msg = parsed
                .get("rsp_msg")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            if debug_print {
                println!("[DEBUG] API 에러 rsp_cd: {}, rsp_msg: {}", rsp_cd, msg);
            }
            return Err(format!("API Error rsp_cd: {}, rsp_msg: {}", rsp_cd, msg).into());
        }
    }

    let stock_list: Vec<StockItem> = match &parsed["t9945OutBlock"] {
        serde_json::Value::Array(_) => {
            match serde_json::from_value(parsed["t9945OutBlock"].clone()) {
                Ok(list) => list,
                Err(e) => {
                    if debug_print {
                        println!("[DEBUG] t9945OutBlock 파싱 실패: {}", e);
                    }
                    return Err(Box::new(e));
                }
            }
        }
        serde_json::Value::Null => {
            if debug_print {
                println!("[DEBUG] t9945OutBlock이 null임");
            }
            Vec::new()
        }
        _ => {
            if debug_print {
                println!(
                    "[DEBUG] t9945OutBlock이 배열/Null이 아님: {:?}",
                    parsed["t9945OutBlock"]
                );
            }
            Vec::new()
        }
    };
    if debug_print {
        println!("[DEBUG] 파싱된 종목 수: {}", stock_list.len());
        for item in stock_list.iter().take(5) {
            println!(
                "[DEBUG] 샘플: {} {} {}",
                item.shcode, item.hname, item.etfchk
            );
        }
    }

    let filtered = stock_list
        .into_iter()
        .filter(|item| {
            // etfchk 필터
            let etfchk_pass = etfchk.map_or(true, |val| item.etfchk == val);
            // etn 필터 (대소문자 무시)
            let etn_pass = match etn_filter {
                Some(true) => item.hname.to_uppercase().contains("ETN"),
                Some(false) => !item.hname.to_uppercase().contains("ETN"),
                None => true,
            };
            etfchk_pass && etn_pass
        })
        .collect::<Vec<_>>();

    let stock_map: HashMap<String, StockItem> = filtered
        .into_iter()
        .map(|mut item| {
            let shcode = std::mem::take(&mut item.shcode);
            (shcode, item)
        })
        .collect();

    if debug_print {
        println!("[DEBUG] 최종 필터링 후 종목 수: {}", stock_map.len());
        for (shcode, stock) in stock_map.iter().take(5) {
            println!(
                "[DEBUG] 최종 샘플: {}: {} {}",
                shcode, stock.hname, stock.etfchk
            );
        }
    }

    Ok(stock_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::oauth::get_access_token;
    use crate::config::AppConfig;
    use std::collections::HashMap;
    use tokio;

    #[tokio::test]
    async fn test_fetch_stock_list_debug() {
        let config = AppConfig::from_env();
        let token = match get_access_token(&config).await {
            Ok(token) => token,
            Err(e) => {
                println!("AccessToken 발급 실패: {}", e);
                return;
            }
        };
        let tr_cd = "t9945";

        // 다양한 조합을 반복적으로 실행하여, 결과가 달라지는 경우를 모두 로그로 남김
        for gubun in &["1", "2"] {
            for etfchk in &[None, Some("0"), Some("1"), Some("999")] {
                for etn_filter in &[None, Some(true), Some(false)] {
                    println!(
                        "\n[TEST] gubun: {:?}, etfchk: {:?}, etn_filter: {:?}",
                        gubun, etfchk, etn_filter
                    );
                    let result =
                        fetch_stock_list(&config, &token, tr_cd, gubun, *etfchk, *etn_filter, true)
                            .await;
                    match &result {
                        Ok(map) => {
                            println!("[RESULT] 종목 수: {}", map.len());
                            for (shcode, stock) in map.iter().take(3) {
                                println!(
                                    "[RESULT] 샘플: {}: {} {}",
                                    shcode, stock.hname, stock.etfchk
                                );
                            }
                        }
                        Err(e) => {
                            println!("[ERROR] fetch_stock_list 실패: {}", e);
                        }
                    }
                }
            }
        }
    }
}
