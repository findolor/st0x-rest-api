use crate::types::common::{Approval, TokenRef};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PeriodUnit {
    Days,
    Hours,
    Minutes,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeployDcaOrderRequest {
    #[schema(example = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")]
    pub input_token: String,
    #[schema(example = "0x4200000000000000000000000000000000000006")]
    pub output_token: String,
    #[schema(example = "1000000")]
    pub budget_amount: String,
    #[schema(example = 4)]
    pub period: u32,
    #[schema(example = "hours")]
    pub period_unit: PeriodUnit,
    #[schema(example = "0.0005")]
    pub start_io: String,
    #[schema(example = "0.0003")]
    pub floor_io: String,
    pub input_vault_id: Option<String>,
    pub output_vault_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeploySolverOrderRequest {
    #[schema(example = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")]
    pub input_token: String,
    #[schema(example = "0x4200000000000000000000000000000000000006")]
    pub output_token: String,
    #[schema(example = "1000000")]
    pub amount: String,
    #[schema(example = "0.0005")]
    pub ioratio: String,
    pub input_vault_id: Option<String>,
    pub output_vault_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeployOrderResponse {
    #[schema(example = "0xDEF171Fe48CF0115B1d80b88dc8eAB59176FEe57")]
    pub to: String,
    #[schema(example = "0xabcdef...")]
    pub data: String,
    #[schema(example = "0")]
    pub value: String,
    pub approvals: Vec<Approval>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab")]
    pub order_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancelTransaction {
    #[schema(example = "0xDEF171Fe48CF0115B1d80b88dc8eAB59176FEe57")]
    pub to: String,
    #[schema(example = "0xabcdef...")]
    pub data: String,
    #[schema(example = "0")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenReturn {
    #[schema(example = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")]
    pub token: String,
    #[schema(example = "USDC")]
    pub symbol: String,
    #[schema(example = "1000000")]
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancelSummary {
    #[schema(example = 2)]
    pub vaults_to_withdraw: u32,
    pub tokens_returned: Vec<TokenReturn>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub transactions: Vec<CancelTransaction>,
    pub summary: CancelSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailsInfo {
    #[serde(rename = "type")]
    #[schema(example = "dca")]
    pub type_: String,
    #[schema(example = "0.0005")]
    pub io_ratio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderTradeEntry {
    #[schema(example = "trade-1")]
    pub id: String,
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab")]
    pub tx_hash: String,
    #[schema(example = "1000000")]
    pub input_amount: String,
    #[schema(example = "500000")]
    pub output_amount: String,
    #[schema(example = 1718452800)]
    pub timestamp: u64,
    #[schema(example = "0x1234567890abcdef1234567890abcdef12345678")]
    pub sender: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetail {
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab")]
    pub order_hash: String,
    #[schema(example = "0x1234567890abcdef1234567890abcdef12345678")]
    pub owner: String,
    pub order_details: OrderDetailsInfo,
    pub input_token: TokenRef,
    pub output_token: TokenRef,
    #[schema(example = "1")]
    pub input_vault_id: String,
    #[schema(example = "2")]
    pub output_vault_id: String,
    #[schema(example = "1000000")]
    pub input_vault_balance: String,
    #[schema(example = "500000")]
    pub output_vault_balance: String,
    #[schema(example = "0.0005")]
    pub io_ratio: String,
    #[schema(example = 1718452800)]
    pub created_at: u64,
    #[schema(example = "0xorderbook")]
    pub orderbook_id: String,
    pub trades: Vec<OrderTradeEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deploy_dca_order_request_serde() {
        let json = r#"{
            "inputToken": "0xabc",
            "outputToken": "0xdef",
            "budgetAmount": "1000000",
            "period": 4,
            "periodUnit": "hours",
            "startIo": "0.0005",
            "floorIo": "0.0003"
        }"#;
        let req: DeployDcaOrderRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.period_unit, PeriodUnit::Hours);
        assert_eq!(req.period, 4);
        assert_eq!(req.budget_amount, "1000000");
        assert!(req.input_vault_id.is_none());
        assert!(req.output_vault_id.is_none());
    }

    #[test]
    fn test_deploy_dca_order_with_vault_ids() {
        let json = r#"{
            "inputToken": "0xabc",
            "outputToken": "0xdef",
            "budgetAmount": "1000000",
            "period": 1,
            "periodUnit": "days",
            "startIo": "0.0005",
            "floorIo": "0.0003",
            "inputVaultId": "42",
            "outputVaultId": "43"
        }"#;
        let req: DeployDcaOrderRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.input_vault_id, Some("42".into()));
        assert_eq!(req.output_vault_id, Some("43".into()));
    }

    #[test]
    fn test_deploy_solver_order_serde() {
        let json = r#"{
            "inputToken": "0xabc",
            "outputToken": "0xdef",
            "amount": "1000000",
            "ioratio": "0.0005"
        }"#;
        let req: DeploySolverOrderRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.ioratio, "0.0005");
        assert!(req.input_vault_id.is_none());
    }

    #[test]
    fn test_deploy_solver_order_with_vault_ids() {
        let json = r#"{
            "inputToken": "0xabc",
            "outputToken": "0xdef",
            "amount": "1000000",
            "ioratio": "0.0005",
            "inputVaultId": "7",
            "outputVaultId": "8"
        }"#;
        let req: DeploySolverOrderRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.input_vault_id, Some("7".into()));
        assert_eq!(req.output_vault_id, Some("8".into()));
    }

    #[test]
    fn test_period_unit_variants() {
        let variants = [
            ("\"minutes\"", PeriodUnit::Minutes),
            ("\"hours\"", PeriodUnit::Hours),
            ("\"days\"", PeriodUnit::Days),
        ];
        for (json, expected) in variants {
            let parsed: PeriodUnit = serde_json::from_str(json).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn test_period_unit_rejects_invalid() {
        let result = serde_json::from_str::<PeriodUnit>("\"seconds\"");
        assert!(result.is_err());
        let result = serde_json::from_str::<PeriodUnit>("\"weeks\"");
        assert!(result.is_err());
        let result = serde_json::from_str::<PeriodUnit>("\"months\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_cancel_order_request_serde() {
        let json = r#"{"orderHash": "0xabc123"}"#;
        let req: CancelOrderRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.order_hash, "0xabc123");
    }

    #[test]
    fn test_cancel_order_response_serde() {
        let resp = CancelOrderResponse {
            transactions: vec![CancelTransaction {
                to: "0xabc".into(),
                data: "0xdef".into(),
                value: "0".into(),
            }],
            summary: CancelSummary {
                vaults_to_withdraw: 2,
                tokens_returned: vec![TokenReturn {
                    token: "0xtoken".into(),
                    symbol: "USDC".into(),
                    amount: "1000000".into(),
                }],
            },
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("transactions"));
        assert!(json.contains("summary"));
        assert!(json.contains("vaultsToWithdraw"));
        assert!(json.contains("tokensReturned"));
    }

    #[test]
    fn test_order_details_info_type_rename() {
        let info = OrderDetailsInfo {
            type_: "dca".into(),
            io_ratio: "0.0005".into(),
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"type\""));
        assert!(!json.contains("\"type_\""));
    }

    #[test]
    fn test_order_detail_serialization() {
        let detail = OrderDetail {
            order_hash: "0xabc".into(),
            owner: "0x123".into(),
            order_details: OrderDetailsInfo {
                type_: "dca".into(),
                io_ratio: "0.0005".into(),
            },
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
            input_vault_id: "1".into(),
            output_vault_id: "2".into(),
            input_vault_balance: "1000000".into(),
            output_vault_balance: "500000".into(),
            io_ratio: "0.0005".into(),
            created_at: 1718452800,
            orderbook_id: "0xorderbook".into(),
            trades: vec![],
        };
        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("orderHash"));
        assert!(json.contains("inputToken"));
        assert!(json.contains("outputToken"));
        assert!(json.contains("orderDetails"));
        assert!(json.contains("inputVaultId"));
        assert!(json.contains("outputVaultId"));
    }
}
