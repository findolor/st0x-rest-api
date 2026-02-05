use crate::types::common::TokenRef;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct OrdersPaginationParams {
    #[field(name = "page")]
    #[serde(default)]
    #[param(example = 1)]
    pub page: Option<u32>,
    #[field(name = "pageSize")]
    #[serde(default = "crate::types::common::default_page_size")]
    #[param(example = 20)]
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderSummary {
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab")]
    pub order_hash: String,
    #[schema(example = "0x1234567890abcdef1234567890abcdef12345678")]
    pub owner: String,
    pub input_token: TokenRef,
    pub output_token: TokenRef,
    #[schema(example = "500000")]
    pub output_vault_balance: String,
    #[schema(example = "0.0005")]
    pub io_ratio: String,
    #[schema(example = 1718452800)]
    pub created_at: u64,
    #[schema(example = "0xorderbook")]
    pub orderbook_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrdersPagination {
    #[schema(example = 1)]
    pub page: u32,
    #[schema(example = 20)]
    pub page_size: u32,
    #[schema(example = 100)]
    pub total_orders: u64,
    #[schema(example = 5)]
    pub total_pages: u64,
    #[schema(example = true)]
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrdersListResponse {
    pub orders: Vec<OrderSummary>,
    pub pagination: OrdersPagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderByTxEntry {
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab")]
    pub order_hash: String,
    #[schema(example = "0x1234567890abcdef1234567890abcdef12345678")]
    pub owner: String,
    #[schema(example = "0xorderbook")]
    pub orderbook_id: String,
    pub input_token: TokenRef,
    pub output_token: TokenRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrdersByTxResponse {
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab")]
    pub tx_hash: String,
    #[schema(example = 12345678)]
    pub block_number: u64,
    #[schema(example = 1718452800)]
    pub timestamp: u64,
    pub orders: Vec<OrderByTxEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_defaults() {
        let json = r#"{}"#;
        let params: OrdersPaginationParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.page_size, Some(20));
        assert!(params.page.is_none());
    }

    #[test]
    fn test_pagination_custom_values() {
        let json = r#"{"pageSize": 50, "page": 3}"#;
        let params: OrdersPaginationParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.page_size, Some(50));
        assert_eq!(params.page, Some(3));
    }

    #[test]
    fn test_orders_list_response_serde() {
        let resp = OrdersListResponse {
            orders: vec![OrderSummary {
                order_hash: "0xabc".into(),
                owner: "0x123".into(),
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
                output_vault_balance: "500000".into(),
                io_ratio: "0.0005".into(),
                created_at: 1718452800,
                orderbook_id: "0xorderbook".into(),
            }],
            pagination: OrdersPagination {
                page: 1,
                page_size: 20,
                total_orders: 1,
                total_pages: 1,
                has_more: false,
            },
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("orderHash"));
        assert!(json.contains("pagination"));
        assert!(json.contains("totalOrders"));
        assert!(json.contains("hasMore"));
        let deserialized: OrdersListResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.orders.len(), 1);
    }

    #[test]
    fn test_orders_by_tx_response_serde() {
        let resp = OrdersByTxResponse {
            tx_hash: "0xtx".into(),
            block_number: 12345678,
            timestamp: 1718452800,
            orders: vec![],
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("txHash"));
        assert!(json.contains("blockNumber"));
        assert!(json.contains("timestamp"));
        let deserialized: OrdersByTxResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.block_number, 12345678);
    }
}
