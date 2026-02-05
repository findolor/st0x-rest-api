use crate::types::common::Approval;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SwapQuoteRequest {
    #[schema(example = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")]
    pub input_token: String,
    #[schema(example = "0x4200000000000000000000000000000000000006")]
    pub output_token: String,
    #[schema(example = "1000000")]
    pub output_amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SwapQuoteResponse {
    #[schema(example = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")]
    pub input_token: String,
    #[schema(example = "0x4200000000000000000000000000000000000006")]
    pub output_token: String,
    #[schema(example = "1000000")]
    pub output_amount: String,
    #[schema(example = "500000000000000")]
    pub estimated_input: String,
    #[schema(example = "0.0005")]
    pub estimated_io_ratio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SwapCalldataRequest {
    #[schema(example = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")]
    pub input_token: String,
    #[schema(example = "0x4200000000000000000000000000000000000006")]
    pub output_token: String,
    #[schema(example = "1000000")]
    pub output_amount: String,
    #[schema(example = "0.0006")]
    pub maximum_io_ratio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SwapCalldataResponse {
    #[schema(example = "0xDEF171Fe48CF0115B1d80b88dc8eAB59176FEe57")]
    pub to: String,
    #[schema(example = "0xabcdef...")]
    pub data: String,
    #[schema(example = "0")]
    pub value: String,
    #[schema(example = "500000000000000")]
    pub estimated_input: String,
    pub approvals: Vec<Approval>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_quote_request_serde() {
        let json = r#"{
            "inputToken": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
            "outputToken": "0x4200000000000000000000000000000000000006",
            "outputAmount": "1000000"
        }"#;
        let req: SwapQuoteRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.output_amount, "1000000");
        assert_eq!(req.input_token, "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913");
    }

    #[test]
    fn test_swap_quote_response_serde() {
        let resp = SwapQuoteResponse {
            input_token: "0xabc".into(),
            output_token: "0xdef".into(),
            output_amount: "1000000".into(),
            estimated_input: "500000000000000".into(),
            estimated_io_ratio: "0.0005".into(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("inputToken"));
        assert!(json.contains("outputToken"));
        assert!(json.contains("outputAmount"));
        assert!(json.contains("estimatedInput"));
        assert!(json.contains("estimatedIoRatio"));
    }

    #[test]
    fn test_swap_calldata_request_serde() {
        let json = r#"{
            "inputToken": "0xabc",
            "outputToken": "0xdef",
            "outputAmount": "100",
            "maximumIoRatio": "0.0006"
        }"#;
        let req: SwapCalldataRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.maximum_io_ratio, "0.0006");
    }

    #[test]
    fn test_swap_calldata_response_serde() {
        let resp = SwapCalldataResponse {
            to: "0xDEF171Fe48CF0115B1d80b88dc8eAB59176FEe57".into(),
            data: "0xabcdef".into(),
            value: "0".into(),
            estimated_input: "500000".into(),
            approvals: vec![],
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: SwapCalldataResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.to, resp.to);
        assert!(deserialized.approvals.is_empty());
    }
}
