use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenRef {
    #[schema(example = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")]
    pub address: String,
    #[schema(example = "USDC")]
    pub symbol: String,
    #[schema(example = 6)]
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Approval {
    #[schema(example = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")]
    pub token: String,
    #[schema(example = "0x1234567890abcdef1234567890abcdef12345678")]
    pub spender: String,
    #[schema(example = "1000000")]
    pub amount: String,
    #[schema(example = "USDC")]
    pub symbol: String,
    #[schema(example = "0xabcdef...")]
    pub approval_data: String,
}

pub fn default_page_size() -> Option<u32> {
    Some(20)
}

pub struct OrderHash(pub String);

impl<'a> rocket::request::FromParam<'a> for OrderHash {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if param.starts_with("0x")
            && param.len() > 2
            && param[2..].chars().all(|c| c.is_ascii_hexdigit())
        {
            Ok(OrderHash(param.to_string()))
        } else {
            Err(param)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_ref_serde() {
        let json = r#"{
            "address": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
            "symbol": "USDC",
            "decimals": 6
        }"#;
        let token: TokenRef = serde_json::from_str(json).unwrap();
        assert_eq!(token.symbol, "USDC");
        assert_eq!(token.decimals, 6);
    }

    #[test]
    fn test_approval_serde() {
        let approval = Approval {
            token: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
            spender: "0x1234".into(),
            amount: "1000000".into(),
            symbol: "USDC".into(),
            approval_data: "0xabcdef".into(),
        };
        let json = serde_json::to_string(&approval).unwrap();
        assert!(json.contains("approvalData"));
        let deserialized: Approval = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.token, approval.token);
    }
}
