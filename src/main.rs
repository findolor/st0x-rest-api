#[macro_use]
extern crate rocket;

mod error;
mod routes;
mod types;

use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::health::get_health,
        routes::tokens::get_tokens,
        routes::swap::post_swap_quote,
        routes::swap::post_swap_calldata,
        routes::order::post_order_dca,
        routes::order::post_order_solver,
        routes::order::get_order,
        routes::order::post_order_cancel,
        routes::orders::get_orders_by_tx,
        routes::orders::get_orders_by_address,
        routes::trades::get_trades_by_tx,
        routes::trades::get_trades_by_address,
    ),
    components(schemas(
        error::ApiErrorResponse,
        error::ApiErrorDetail,
        types::health::HealthResponse,
        types::common::TokenRef,
        types::common::Approval,
        types::tokens::TokenInfo,
        types::tokens::TokenListResponse,
        types::swap::SwapQuoteRequest,
        types::swap::SwapQuoteResponse,
        types::swap::SwapCalldataRequest,
        types::swap::SwapCalldataResponse,
        types::order::PeriodUnit,
        types::order::DeployDcaOrderRequest,
        types::order::DeploySolverOrderRequest,
        types::order::DeployOrderResponse,
        types::order::CancelOrderRequest,
        types::order::CancelOrderResponse,
        types::order::CancelTransaction,
        types::order::CancelSummary,
        types::order::TokenReturn,
        types::order::OrderDetailsInfo,
        types::order::OrderTradeEntry,
        types::order::OrderDetail,
        types::orders::OrderSummary,
        types::orders::OrdersPagination,
        types::orders::OrdersListResponse,
        types::orders::OrderByTxEntry,
        types::orders::OrdersByTxResponse,
        types::trades::TradeByAddress,
        types::trades::TradesPagination,
        types::trades::TradesByAddressResponse,
        types::trades::TradeRequest,
        types::trades::TradeResult,
        types::trades::TradeByTxEntry,
        types::trades::TradesTotals,
        types::trades::TradesByTxResponse,
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Tokens", description = "Token information endpoints"),
        (name = "Swap", description = "Swap quote and calldata endpoints"),
        (name = "Order", description = "Order deployment and management endpoints"),
        (name = "Orders", description = "Order listing and query endpoints"),
        (name = "Trades", description = "Trade listing and query endpoints"),
    ),
    info(
        title = "st0x REST API",
        version = "0.1.0",
        description = "REST API for st0x orderbook operations",
    )
)]
struct ApiDoc;

fn configure_cors() -> CorsOptions {
    let allowed_methods: AllowedMethods = ["Get", "Post", "Options"]
        .iter()
        .map(|s| std::str::FromStr::from_str(s).unwrap())
        .collect();

    CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: false,
        ..Default::default()
    }
}

fn rocket() -> rocket::Rocket<rocket::Build> {
    let cors = configure_cors().to_cors().expect("CORS configuration failed");

    rocket::build()
        .mount("/", routes::health::routes())
        .mount("/v1/tokens", routes::tokens::routes())
        .mount("/v1/swap", routes::swap::routes())
        .mount("/v1/order", routes::order::routes())
        .mount("/v1/orders", routes::orders::routes())
        .mount("/v1/trades", routes::trades::routes())
        .mount(
            "/",
            SwaggerUi::new("/swagger/<tail..>").url("/api-doc/openapi.json", ApiDoc::openapi()),
        )
        .attach(cors)
}

#[launch]
fn launch() -> _ {
    rocket()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::{ContentType, Header, Method, Status};
    use rocket::local::blocking::Client;

    fn client() -> Client {
        Client::tracked(rocket()).expect("valid rocket instance")
    }

    fn assert_cors_preflight(client: &Client, path: &str) {
        let response = client
            .req(Method::Options, path)
            .header(Header::new("Origin", "http://localhost:3000"))
            .header(Header::new("Access-Control-Request-Method", "POST"))
            .header(Header::new(
                "Access-Control-Request-Headers",
                "Content-Type",
            ))
            .dispatch();
        assert_ne!(response.status(), Status::NotFound);
        assert!(response
            .headers()
            .get_one("Access-Control-Allow-Origin")
            .is_some());
    }

    #[test]
    fn test_cors_preflight_tokens() {
        let client = client();
        assert_cors_preflight(&client, "/v1/tokens");
    }

    #[test]
    fn test_cors_preflight_swap() {
        let client = client();
        assert_cors_preflight(&client, "/v1/swap/quote");
        assert_cors_preflight(&client, "/v1/swap/calldata");
    }

    #[test]
    fn test_cors_preflight_order() {
        let client = client();
        assert_cors_preflight(&client, "/v1/order/dca");
        assert_cors_preflight(&client, "/v1/order/solver");
        assert_cors_preflight(&client, "/v1/order/cancel");
    }

    #[test]
    fn test_cors_preflight_orders() {
        let client = client();
        assert_cors_preflight(&client, "/v1/orders/tx/0x123");
        assert_cors_preflight(&client, "/v1/orders/0xaddr");
    }

    #[test]
    fn test_cors_preflight_trades() {
        let client = client();
        assert_cors_preflight(&client, "/v1/trades/tx/0x123");
        assert_cors_preflight(&client, "/v1/trades/0xaddr");
    }

    #[test]
    fn test_health_endpoint() {
        let client = client();
        let response = client.get("/health").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body: serde_json::Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
        assert_eq!(body["status"], "ok");
    }

    fn assert_missing_field_422(client: &Client, path: &str, json: &str) {
        let response = client
            .post(path)
            .header(ContentType::JSON)
            .body(json)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_swap_quote_missing_field() {
        let client = client();
        assert_missing_field_422(
            &client,
            "/v1/swap/quote",
            r#"{"inputToken": "0x1", "outputToken": "0x2"}"#,
        );
    }

    #[test]
    fn test_swap_calldata_missing_field() {
        let client = client();
        assert_missing_field_422(
            &client,
            "/v1/swap/calldata",
            r#"{"inputToken": "0x1", "outputToken": "0x2"}"#,
        );
    }

    #[test]
    fn test_order_dca_missing_field() {
        let client = client();
        assert_missing_field_422(
            &client,
            "/v1/order/dca",
            r#"{"inputToken": "0x1"}"#,
        );
    }

    #[test]
    fn test_order_solver_missing_field() {
        let client = client();
        assert_missing_field_422(
            &client,
            "/v1/order/solver",
            r#"{"inputToken": "0x1"}"#,
        );
    }

    #[test]
    fn test_order_cancel_missing_field() {
        let client = client();
        assert_missing_field_422(
            &client,
            "/v1/order/cancel",
            r#"{}"#,
        );
    }

    #[test]
    fn test_swagger_ui_returns_html() {
        let client = client();
        let response = client.get("/swagger/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("html"));
    }

    fn get_openapi_json(client: &Client) -> serde_json::Value {
        let response = client.get("/api-doc/openapi.json").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        serde_json::from_str(&body).unwrap()
    }

    #[test]
    fn test_openapi_json_valid_spec() {
        let client = client();
        let spec = get_openapi_json(&client);
        assert!(spec["openapi"].as_str().unwrap().starts_with("3."));
        assert_eq!(spec["info"]["title"].as_str().unwrap(), "st0x REST API");
        assert_eq!(spec["info"]["version"].as_str().unwrap(), "0.1.0");
    }

    #[test]
    fn test_openapi_json_contains_all_paths() {
        let client = client();
        let spec = get_openapi_json(&client);
        let paths = spec["paths"].as_object().unwrap();

        let expected_paths = [
            "/health",
            "/v1/tokens",
            "/v1/swap/quote",
            "/v1/swap/calldata",
            "/v1/order/dca",
            "/v1/order/solver",
            "/v1/order/{order_hash}",
            "/v1/order/cancel",
            "/v1/orders/tx/{tx_hash}",
            "/v1/orders/{address}",
            "/v1/trades/tx/{tx_hash}",
            "/v1/trades/{address}",
        ];

        for path in &expected_paths {
            assert!(
                paths.contains_key(*path),
                "Missing path: {path}. Found: {:?}",
                paths.keys().collect::<Vec<_>>()
            );
        }
        assert_eq!(paths.len(), expected_paths.len());
    }

    #[test]
    fn test_openapi_json_contains_all_schemas() {
        let client = client();
        let spec = get_openapi_json(&client);
        let schemas = spec["components"]["schemas"].as_object().unwrap();

        let expected_schemas = [
            "ApiErrorResponse",
            "ApiErrorDetail",
            "HealthResponse",
            "TokenRef",
            "Approval",
            "TokenInfo",
            "TokenListResponse",
            "SwapQuoteRequest",
            "SwapQuoteResponse",
            "SwapCalldataRequest",
            "SwapCalldataResponse",
            "PeriodUnit",
            "DeployDcaOrderRequest",
            "DeploySolverOrderRequest",
            "DeployOrderResponse",
            "CancelOrderRequest",
            "CancelOrderResponse",
            "CancelTransaction",
            "CancelSummary",
            "TokenReturn",
            "OrderDetailsInfo",
            "OrderTradeEntry",
            "OrderDetail",
            "OrderSummary",
            "OrdersPagination",
            "OrdersListResponse",
            "OrderByTxEntry",
            "OrdersByTxResponse",
            "TradeByAddress",
            "TradesPagination",
            "TradesByAddressResponse",
            "TradeRequest",
            "TradeResult",
            "TradeByTxEntry",
            "TradesTotals",
            "TradesByTxResponse",
        ];

        for schema in &expected_schemas {
            assert!(
                schemas.contains_key(*schema),
                "Missing schema: {schema}. Found: {:?}",
                schemas.keys().collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_openapi_json_contains_response_codes() {
        let client = client();
        let spec = get_openapi_json(&client);
        let paths = spec["paths"].as_object().unwrap();

        for (path, methods) in paths {
            let methods = methods.as_object().unwrap();
            for (method, operation) in methods {
                let responses = operation["responses"].as_object();
                assert!(
                    responses.is_some(),
                    "Missing responses for {method} {path}"
                );
                let responses = responses.unwrap();
                assert!(
                    !responses.is_empty(),
                    "Empty responses for {method} {path}"
                );
                assert!(
                    responses.contains_key("200") || responses.contains_key("202"),
                    "Missing success response for {method} {path}"
                );
            }
        }
    }

    #[test]
    fn test_openapi_field_descriptions_present() {
        let client = client();
        let spec = get_openapi_json(&client);
        let schemas = &spec["components"]["schemas"];

        let token_info = &schemas["TokenInfo"];
        assert!(token_info["properties"]["address"].is_object());
        assert!(token_info["properties"]["symbol"].is_object());
        assert!(token_info["properties"]["decimals"].is_object());
        assert!(token_info["properties"]["ISIN"].is_object());

        let swap_quote = &schemas["SwapQuoteRequest"];
        assert!(swap_quote["properties"]["inputToken"].is_object());
        assert!(swap_quote["properties"]["outputAmount"].is_object());
    }

    fn get_schema_ref(val: &serde_json::Value) -> &str {
        val["$ref"].as_str().unwrap_or("")
    }

    fn schema_ref(name: &str) -> String {
        format!("#/components/schemas/{name}")
    }

    #[test]
    fn test_openapi_response_schema_references() {
        let client = client();
        let spec = get_openapi_json(&client);
        let paths = &spec["paths"];

        let cases: Vec<(&str, &str, &str, &str)> = vec![
            ("/health", "get", "200", "HealthResponse"),
            ("/v1/tokens", "get", "200", "TokenListResponse"),
            ("/v1/swap/quote", "post", "200", "SwapQuoteResponse"),
            ("/v1/swap/calldata", "post", "200", "SwapCalldataResponse"),
            ("/v1/order/dca", "post", "200", "DeployOrderResponse"),
            ("/v1/order/solver", "post", "200", "DeployOrderResponse"),
            ("/v1/order/{order_hash}", "get", "200", "OrderDetail"),
            ("/v1/order/cancel", "post", "200", "CancelOrderResponse"),
            ("/v1/orders/tx/{tx_hash}", "get", "200", "OrdersByTxResponse"),
            ("/v1/orders/{address}", "get", "200", "OrdersListResponse"),
            ("/v1/trades/tx/{tx_hash}", "get", "200", "TradesByTxResponse"),
            ("/v1/trades/{address}", "get", "200", "TradesByAddressResponse"),
        ];

        for (path, method, status, expected_schema) in &cases {
            let response_schema = &paths[path][method]["responses"][status]["content"]
                ["application/json"]["schema"];
            assert_eq!(
                get_schema_ref(response_schema),
                schema_ref(expected_schema),
                "{method} {path} -> {status} should reference {expected_schema}"
            );
        }
    }

    #[test]
    fn test_openapi_request_body_schema_references() {
        let client = client();
        let spec = get_openapi_json(&client);
        let paths = &spec["paths"];

        let cases: Vec<(&str, &str)> = vec![
            ("/v1/swap/quote", "SwapQuoteRequest"),
            ("/v1/swap/calldata", "SwapCalldataRequest"),
            ("/v1/order/dca", "DeployDcaOrderRequest"),
            ("/v1/order/solver", "DeploySolverOrderRequest"),
            ("/v1/order/cancel", "CancelOrderRequest"),
        ];

        for (path, expected_schema) in &cases {
            let body_schema =
                &paths[path]["post"]["requestBody"]["content"]["application/json"]["schema"];
            assert_eq!(
                get_schema_ref(body_schema),
                schema_ref(expected_schema),
                "POST {path} requestBody should reference {expected_schema}"
            );
        }
    }

    fn get_required_fields(spec: &serde_json::Value, schema_name: &str) -> Vec<String> {
        let schema = &spec["components"]["schemas"][schema_name];
        match schema["required"].as_array() {
            Some(arr) => arr.iter().map(|v| v.as_str().unwrap().to_string()).collect(),
            None => vec![],
        }
    }

    #[test]
    fn test_openapi_required_fields_request_types() {
        let client = client();
        let spec = get_openapi_json(&client);

        let swap_quote_required = get_required_fields(&spec, "SwapQuoteRequest");
        assert!(swap_quote_required.contains(&"inputToken".to_string()));
        assert!(swap_quote_required.contains(&"outputToken".to_string()));
        assert!(swap_quote_required.contains(&"outputAmount".to_string()));
        assert_eq!(swap_quote_required.len(), 3);

        let swap_calldata_required = get_required_fields(&spec, "SwapCalldataRequest");
        assert!(swap_calldata_required.contains(&"inputToken".to_string()));
        assert!(swap_calldata_required.contains(&"outputToken".to_string()));
        assert!(swap_calldata_required.contains(&"outputAmount".to_string()));
        assert!(swap_calldata_required.contains(&"maximumIoRatio".to_string()));
        assert_eq!(swap_calldata_required.len(), 4);

        let dca_required = get_required_fields(&spec, "DeployDcaOrderRequest");
        for field in &[
            "inputToken",
            "outputToken",
            "budgetAmount",
            "period",
            "periodUnit",
            "startIo",
            "floorIo",
        ] {
            assert!(
                dca_required.contains(&field.to_string()),
                "DeployDcaOrderRequest missing required field: {field}"
            );
        }
        assert!(
            !dca_required.contains(&"inputVaultId".to_string()),
            "inputVaultId should be optional in DeployDcaOrderRequest"
        );
        assert!(
            !dca_required.contains(&"outputVaultId".to_string()),
            "outputVaultId should be optional in DeployDcaOrderRequest"
        );

        let solver_required = get_required_fields(&spec, "DeploySolverOrderRequest");
        for field in &["inputToken", "outputToken", "amount", "ioratio"] {
            assert!(
                solver_required.contains(&field.to_string()),
                "DeploySolverOrderRequest missing required field: {field}"
            );
        }
        assert!(
            !solver_required.contains(&"inputVaultId".to_string()),
            "inputVaultId should be optional in DeploySolverOrderRequest"
        );
        assert!(
            !solver_required.contains(&"outputVaultId".to_string()),
            "outputVaultId should be optional in DeploySolverOrderRequest"
        );

        let cancel_required = get_required_fields(&spec, "CancelOrderRequest");
        assert!(cancel_required.contains(&"orderHash".to_string()));
        assert_eq!(cancel_required.len(), 1);
    }

    #[test]
    fn test_openapi_required_fields_response_types() {
        let client = client();
        let spec = get_openapi_json(&client);

        let deploy_resp = get_required_fields(&spec, "DeployOrderResponse");
        for field in &["to", "data", "value", "approvals"] {
            assert!(
                deploy_resp.contains(&field.to_string()),
                "DeployOrderResponse missing required field: {field}"
            );
        }

        let cancel_resp = get_required_fields(&spec, "CancelOrderResponse");
        assert!(cancel_resp.contains(&"transactions".to_string()));
        assert!(cancel_resp.contains(&"summary".to_string()));

        let orders_list = get_required_fields(&spec, "OrdersListResponse");
        assert!(orders_list.contains(&"orders".to_string()));
        assert!(orders_list.contains(&"pagination".to_string()));

        let orders_by_tx = get_required_fields(&spec, "OrdersByTxResponse");
        for field in &["txHash", "blockNumber", "timestamp", "orders"] {
            assert!(
                orders_by_tx.contains(&field.to_string()),
                "OrdersByTxResponse missing required field: {field}"
            );
        }

        let trades_by_addr = get_required_fields(&spec, "TradesByAddressResponse");
        assert!(trades_by_addr.contains(&"trades".to_string()));
        assert!(trades_by_addr.contains(&"pagination".to_string()));

        let trades_by_tx = get_required_fields(&spec, "TradesByTxResponse");
        for field in &["txHash", "blockNumber", "timestamp", "sender", "trades", "totals"] {
            assert!(
                trades_by_tx.contains(&field.to_string()),
                "TradesByTxResponse missing required field: {field}"
            );
        }

        let trade_by_addr = get_required_fields(&spec, "TradeByAddress");
        assert!(
            !trade_by_addr.contains(&"orderHash".to_string()),
            "orderHash should be optional in TradeByAddress"
        );
        for field in &[
            "txHash",
            "inputAmount",
            "outputAmount",
            "inputToken",
            "outputToken",
            "timestamp",
            "blockNumber",
        ] {
            assert!(
                trade_by_addr.contains(&field.to_string()),
                "TradeByAddress missing required field: {field}"
            );
        }
    }
}
