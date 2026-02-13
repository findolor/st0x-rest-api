use crate::auth::AuthenticatedKey;
use crate::error::{ApiError, ApiErrorResponse};
use crate::fairings::{GlobalRateLimit, TracingSpan};
use crate::types::tokens::{RemoteTokenList, TokenInfo, TokenListResponse};
use rocket::serde::json::Json;
use rocket::{Route, State};
use tracing::Instrument;

const TOKEN_LIST_URL: &str = "https://raw.githubusercontent.com/S01-Issuer/st0x-tokens/ad1a637a79d5a220ad089aecdc5b7239d3473f6e/src/st0xTokens.json";
const TARGET_CHAIN_ID: u32 = 8453;
const HARDCODED_ISIN: &str = "US0000000000";

pub(crate) struct TokenListUrl(pub String);

impl Default for TokenListUrl {
    fn default() -> Self {
        Self(TOKEN_LIST_URL.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
enum TokenError {
    #[error("failed to fetch token list: {0}")]
    Fetch(reqwest::Error),
    #[error("failed to deserialize token list: {0}")]
    Deserialize(reqwest::Error),
    #[error("token list returned non-200 status: {0}")]
    BadStatus(reqwest::StatusCode),
}

impl From<TokenError> for ApiError {
    fn from(e: TokenError) -> Self {
        tracing::error!(error = %e, "token list fetch failed");
        ApiError::Internal("failed to retrieve token list".into())
    }
}

#[utoipa::path(
    get,
    path = "/v1/tokens",
    tag = "Tokens",
    security(("basicAuth" = [])),
    responses(
        (status = 200, description = "List of supported tokens", body = TokenListResponse),
        (status = 401, description = "Unauthorized", body = ApiErrorResponse),
        (status = 429, description = "Rate limited", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
#[get("/")]
pub async fn get_tokens(
    _global: GlobalRateLimit,
    _key: AuthenticatedKey,
    span: TracingSpan,
    token_list_url: &State<TokenListUrl>,
) -> Result<Json<TokenListResponse>, ApiError> {
    let url = token_list_url.0.clone();
    async move {
        tracing::info!("request received");

        let response = reqwest::get(&url)
            .await
            .map_err(TokenError::Fetch)?;

        let status = response.status();
        if !status.is_success() {
            return Err(TokenError::BadStatus(status).into());
        }

        let remote: RemoteTokenList = response
            .json()
            .await
            .map_err(TokenError::Deserialize)?;

        let tokens: Vec<TokenInfo> = remote
            .tokens
            .into_iter()
            .filter(|t| t.chain_id == TARGET_CHAIN_ID)
            .map(|t| TokenInfo {
                address: t.address,
                symbol: t.symbol,
                name: t.name,
                isin: HARDCODED_ISIN.into(),
                decimals: t.decimals,
            })
            .collect();

        tracing::info!(count = tokens.len(), "returning tokens");
        Ok(Json(TokenListResponse { tokens }))
    }
    .instrument(span.0)
    .await
}

pub fn routes() -> Vec<Route> {
    rocket::routes![get_tokens]
}
