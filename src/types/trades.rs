use crate::types::common::TokenRef;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct TradesPaginationParams {
    #[field(name = "page")]
    #[serde(default)]
    #[param(example = 1)]
    pub page: Option<u32>,
    #[field(name = "pageSize")]
    #[serde(default = "crate::types::common::default_page_size")]
    #[param(example = 20)]
    pub page_size: Option<u32>,
    #[field(name = "startTime")]
    #[param(example = 1718452800)]
    pub start_time: Option<u64>,
    #[field(name = "endTime")]
    #[param(example = 1718539200)]
    pub end_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TradeByAddress {
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab")]
    pub tx_hash: String,
    #[schema(example = "1000000")]
    pub input_amount: String,
    #[schema(example = "500000")]
    pub output_amount: String,
    pub input_token: TokenRef,
    pub output_token: TokenRef,
    pub order_hash: Option<String>,
    #[schema(example = 1718452800)]
    pub timestamp: u64,
    #[schema(example = 12345678)]
    pub block_number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TradesPagination {
    #[schema(example = 1)]
    pub page: u32,
    #[schema(example = 20)]
    pub page_size: u32,
    #[schema(example = 100)]
    pub total_trades: u64,
    #[schema(example = 5)]
    pub total_pages: u64,
    #[schema(example = true)]
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TradesByAddressResponse {
    pub trades: Vec<TradeByAddress>,
    pub pagination: TradesPagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TradeRequest {
    #[schema(example = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")]
    pub input_token: String,
    #[schema(example = "0x4200000000000000000000000000000000000006")]
    pub output_token: String,
    #[schema(example = "1000000")]
    pub maximum_input: String,
    #[schema(example = "0.0006")]
    pub maximum_io_ratio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TradeResult {
    #[schema(example = "900000")]
    pub input_amount: String,
    #[schema(example = "500000")]
    pub output_amount: String,
    #[schema(example = "0.00055")]
    pub actual_io_ratio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TradeByTxEntry {
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab")]
    pub order_hash: String,
    #[schema(example = "0x1234567890abcdef1234567890abcdef12345678")]
    pub order_owner: String,
    pub request: TradeRequest,
    pub result: TradeResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TradesTotals {
    #[schema(example = "900000")]
    pub total_input_amount: String,
    #[schema(example = "500000")]
    pub total_output_amount: String,
    #[schema(example = "0.00055")]
    pub average_io_ratio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TradesByTxResponse {
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab")]
    pub tx_hash: String,
    #[schema(example = 12345678)]
    pub block_number: u64,
    #[schema(example = 1718452800)]
    pub timestamp: u64,
    #[schema(example = "0x1234567890abcdef1234567890abcdef12345678")]
    pub sender: String,
    pub trades: Vec<TradeByTxEntry>,
    pub totals: TradesTotals,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trades_pagination_defaults() {
        let json = r#"{}"#;
        let params: TradesPaginationParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.page_size, Some(20));
        assert!(params.page.is_none());
        assert!(params.start_time.is_none());
        assert!(params.end_time.is_none());
    }

    #[test]
    fn test_trades_pagination_with_times() {
        let json = r#"{
            "pageSize": 10,
            "page": 1,
            "startTime": 1718452800,
            "endTime": 1718539200
        }"#;
        let params: TradesPaginationParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.start_time, Some(1718452800));
        assert_eq!(params.end_time, Some(1718539200));
    }

    #[test]
    fn test_trade_by_address_serde() {
        let trade = TradeByAddress {
            tx_hash: "0xtx".into(),
            input_amount: "1000000".into(),
            output_amount: "500000".into(),
            input_token: TokenRef {
                address: "0xtoken_in".into(),
                symbol: "USDC".into(),
                decimals: 6,
            },
            output_token: TokenRef {
                address: "0xtoken_out".into(),
                symbol: "WETH".into(),
                decimals: 18,
            },
            order_hash: Some("0xorder".into()),
            timestamp: 1718452800,
            block_number: 12345678,
        };
        let json = serde_json::to_string(&trade).unwrap();
        assert!(json.contains("txHash"));
        assert!(json.contains("inputAmount"));
        assert!(json.contains("outputAmount"));
        assert!(json.contains("inputToken"));
        assert!(json.contains("outputToken"));
        assert!(json.contains("blockNumber"));
    }

    #[test]
    fn test_trades_by_address_response_serde() {
        let resp = TradesByAddressResponse {
            trades: vec![],
            pagination: TradesPagination {
                page: 1,
                page_size: 20,
                total_trades: 0,
                total_pages: 0,
                has_more: false,
            },
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("totalTrades"));
        assert!(json.contains("hasMore"));
        let deserialized: TradesByAddressResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.pagination.total_trades, 0);
    }

    #[test]
    fn test_trades_by_tx_response_serde() {
        let resp = TradesByTxResponse {
            tx_hash: "0xtx".into(),
            block_number: 12345678,
            timestamp: 1718452800,
            sender: "0xsender".into(),
            trades: vec![TradeByTxEntry {
                order_hash: "0xorder".into(),
                order_owner: "0xowner".into(),
                request: TradeRequest {
                    input_token: "0xin".into(),
                    output_token: "0xout".into(),
                    maximum_input: "1000000".into(),
                    maximum_io_ratio: "0.0006".into(),
                },
                result: TradeResult {
                    input_amount: "900000".into(),
                    output_amount: "500000".into(),
                    actual_io_ratio: "0.00055".into(),
                },
            }],
            totals: TradesTotals {
                total_input_amount: "900000".into(),
                total_output_amount: "500000".into(),
                average_io_ratio: "0.00055".into(),
            },
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("txHash"));
        assert!(json.contains("blockNumber"));
        assert!(json.contains("sender"));
        assert!(json.contains("totals"));
        assert!(json.contains("totalInputAmount"));
        assert!(json.contains("averageIoRatio"));
        let deserialized: TradesByTxResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.trades.len(), 1);
    }
}
