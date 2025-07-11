// LS증권 실시간 시세 WebSocket 관련 상수 정의
// 운영/모의투자 도메인, 엔드포인트, tr_type, 예시 tr_cd 등

/// 운영 환경 WebSocket 도메인
pub const LS_WS_DOMAIN_PROD: &str = "wss://openapi.ls-sec.co.kr:9443";
/// 모의투자 환경 WebSocket 도메인
pub const LS_WS_DOMAIN_DEMO: &str = "wss://openapi.ls-sec.co.kr:29443";
/// WebSocket 엔드포인트
pub const LS_WS_ENDPOINT: &str = "/websocket";

//------------------------------------------------------------------------------
/// 1. 계좌 등록 tr_type 값
pub const LS_WS_TR_TYPE_ACCOUNT_REGISTER: &str = "1";
/// 2. 계좌 해제 tr_type 값
pub const LS_WS_TR_TYPE_ACCOUNT_UNREGISTER: &str = "2";
/// 3. 실시간 시세 등록 tr_type 값
pub const LS_WS_TR_TYPE_REGISTER: &str = "3";
/// 4. 실시간 시세 해제 tr_type 값
pub const LS_WS_TR_TYPE_UNREGISTER: &str = "4";

//----------[주식 실시간 시세]--------------------------------------------------------------------
pub const LS_WS_TR_CD_ETF_ORDERBOOK: &str = "B7_";
/// ETF 호가잔량
pub const LS_WS_TR_CD_KOSPI_AFTER_HOURS_ORDERBOOK: &str = "DH1";
/// KOSPI 시간외 단일가 호가잔량
pub const LS_WS_TR_CD_KOSDAQ_AFTER_HOURS_ORDERBOOK: &str = "DHA";
/// KOSDAQ 시간외 단일가 호가잔량
pub const LS_WS_TR_CD_KOSDAQ_AFTER_HOURS_EXECUTION: &str = "DK3";
/// KOSDAQ 시간외 단일가 체결
pub const LS_WS_TR_CD_KOSPI_AFTER_HOURS_EXECUTION: &str = "DS3";
/// KOSPI 시간외 단일가 체결
pub const LS_WS_TR_CD_AFTER_HOURS_VI_RELEASE: &str = "DVI";
/// 시간외 단일가 VI 발동 해제
pub const LS_WS_TR_CD_KOSPI_ORDERBOOK: &str = "H1_";
/// KOSPI 호가잔량
pub const LS_WS_TR_CD_KOSPI_BEFORE_MARKET_ORDERBOOK: &str = "H2_";
/// KOSPI 장전 시간외 호가잔량
pub const LS_WS_TR_CD_KOSDAQ_ORDERBOOK: &str = "HA_";
/// KOSDAQ 호가잔량
pub const LS_WS_TR_CD_KOSDAQ_BEFORE_MARKET_ORDERBOOK: &str = "HB_";
/// KOSDAQ 장전 시간외 호가잔량
pub const LS_WS_TR_CD_KOSPI_ETF_NAV: &str = "I5_";
/// 코스피 ETF 종목 실시간 NAV
pub const LS_WS_TR_CD_INDEX: &str = "IJ_";
/// 지수
pub const LS_WS_TR_CD_KOSPI_BROKER: &str = "K1_";
/// KOSPI 거래원
pub const LS_WS_TR_CD_KOSDAQ_EXECUTION: &str = "K3_";
/// KOSDAQ 체결
pub const LS_WS_TR_CD_KOSDAQ_PROG_STOCK: &str = "KH_";
/// KOSDAQ 프로그램매매 종목별
pub const LS_WS_TR_CD_KOSDAQ_PROG_TOTAL: &str = "KM_";
/// KOSDAQ 프로그램매매 전체집계
pub const LS_WS_TR_CD_KOSDAQ_PRIORITY_ORDERBOOK: &str = "KS_";
/// KOSDAQ 우선호가
pub const LS_WS_TR_CD_KOSDAQ_BROKER: &str = "OK_";
/// KOSDAQ 거래원
pub const LS_WS_TR_CD_KOSPI_PROG_STOCK: &str = "PH_";
/// KOSPI 프로그램매매 종목별
pub const LS_WS_TR_CD_KOSPI_PROG_TOTAL: &str = "PM_";
/// KOSPI 프로그램매매 전체집계
pub const LS_WS_TR_CD_KOSPI_PRIORITY_ORDERBOOK: &str = "S2_";
/// KOSPI 우선호가
pub const LS_WS_TR_CD_KOSPI_EXECUTION: &str = "S3_";
/// KOSPI 체결
pub const LS_WS_TR_CD_KOSPI_TREND: &str = "S4_";
/// KOSPI 기세
pub const LS_WS_TR_CD_ORDER_RECEIVE: &str = "SC0";
/// 주식 주문 접수
pub const LS_WS_TR_CD_ORDER_EXECUTION: &str = "SC1";
/// 주식 주문 체결
pub const LS_WS_TR_CD_ORDER_MODIFY: &str = "SC2";
/// 주식 주문 정정
pub const LS_WS_TR_CD_ORDER_CANCEL: &str = "SC3";
/// 주식 주문 취소
pub const LS_WS_TR_CD_ORDER_REJECT: &str = "SC4";
/// 주식 주문 거부
pub const LS_WS_TR_CD_LIMIT_NEAR_ENTER: &str = "SHC";
/// 상/하한가 근접 진입
pub const LS_WS_TR_CD_LIMIT_NEAR_LEAVE: &str = "SHD";
/// 상/하한가 근접 이탈
pub const LS_WS_TR_CD_LIMIT_ENTER: &str = "SHI";
/// 상/하한가 진입
pub const LS_WS_TR_CD_LIMIT_LEAVE: &str = "SHO";
/// 상/하한가 이탈
pub const LS_WS_TR_CD_VI_RELEASE: &str = "VI_";
/// VI 발동 해제
pub const LS_WS_TR_CD_EXPECTED_INDEX: &str = "YJ_";
/// 예상지수
pub const LS_WS_TR_CD_KOSDAQ_EXPECTED_EXECUTION: &str = "YK3";
/// KOSDAQ 예상 체결
pub const LS_WS_TR_CD_KOSPI_EXPECTED_EXECUTION: &str = "YS3";
/// KOSPI 예상 체결
pub const LS_WS_TR_CD_NEW_ELW_SENSITIVITY: &str = "ESN";
/// 뉴 ELW 투자지표 민감도
pub const LS_WS_TR_CD_ELW_BEFORE_MARKET_ORDERBOOK: &str = "h2_";
/// ELW 장전 시간외 호가잔량
pub const LS_WS_TR_CD_ELW_ORDERBOOK: &str = "h3_";
/// ELW 호가잔량
pub const LS_WS_TR_CD_ELW_BROKER: &str = "k1_";
/// ELW 거래원
pub const LS_WS_TR_CD_ELW_PRIORITY_ORDERBOOK: &str = "s2_";
/// ELW 우선호가
pub const LS_WS_TR_CD_ELW_EXECUTION: &str = "s3_";
/// ELW 체결
pub const LS_WS_TR_CD_ELW_TREND: &str = "s4_";
/// ELW 기세
pub const LS_WS_TR_CD_ELW_EXPECTED_EXECUTION: &str = "Ys3";
/// ELW 예상 체결
pub const LS_WS_TR_CD_NXT_EXECUTION: &str = "NS3";
/// (NXT) 체결
pub const LS_WS_TR_CD_NXT_ORDERBOOK: &str = "NH1";
/// (NXT) 호가잔량
pub const LS_WS_TR_CD_NXT_PRIORITY_ORDERBOOK: &str = "NS2";
/// (NXT) 우선호가
pub const LS_WS_TR_CD_NXT_EXPECTED_EXECUTION: &str = "NYS";
/// (NXT) 예상체결
pub const LS_WS_TR_CD_NXT_VI_RELEASE: &str = "NVI";
/// (NXT) VI 발동 해제
pub const LS_WS_TR_CD_NXT_BROKER: &str = "NK1";
/// (NXT) 거래원
pub const LS_WS_TR_CD_NXT_PROG_STOCK: &str = "NPH";
/// (NXT) 프로그램매매 종목별
pub const LS_WS_TR_CD_NXT_PROG_TOTAL: &str = "NPM";
/// (NXT) 프로그램매매 전체집계
pub const LS_WS_TR_CD_NXT_INVESTOR_TREND: &str = "NBT";
/// (NXT) 시간대별 투자자 매매추이
pub const LS_WS_TR_CD_NXT_SECTOR_INVESTOR_STATUS: &str = "NBM";
/// (NXT) 업종별 투자자별 매매현황
pub const LS_WS_TR_CD_UNI_EXECUTION: &str = "US3";
/// (통합) 체결
pub const LS_WS_TR_CD_UNI_ORDERBOOK: &str = "UH1";
/// (통합) 호가잔량
pub const LS_WS_TR_CD_UNI_PRIORITY_ORDERBOOK: &str = "US2";
/// (통합) 우선호가
pub const LS_WS_TR_CD_UNI_EXPECTED_EXECUTION: &str = "UYS";
/// (통합) 예상체결
pub const LS_WS_TR_CD_UNI_PROG_STOCK: &str = "UPH";
/// (통합) 프로그램매매 종목별
pub const LS_WS_TR_CD_UNI_BROKER: &str = "UK1";
/// (통합) 거래원
pub const LS_WS_TR_CD_UNI_INVESTOR_TREND: &str = "UBT";
/// (통합) 시간대별 투자자 매매추이
pub const LS_WS_TR_CD_UNI_SECTOR_INVESTOR_STATUS: &str = "UBM";
/// (통합) 업종별 투자자별 매매현황
pub const LS_WS_TR_CD_UNI_PROG_TOTAL: &str = "UPM";
/// (통합) 프로그램매매 전체집계
pub const LS_WS_TR_CD_UNI_VI_RELEASE: &str = "UVI";
/// (통합) VI 발동 해제
/// API 사용자 조건검색 실시간
pub const LS_WS_TR_CD_USER_CONDITION_SEARCH: &str = "AFR";
