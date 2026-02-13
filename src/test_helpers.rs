use base64::Engine;
use rocket::local::asynchronous::Client;

pub(crate) async fn client() -> Client {
    let rate_limiter = crate::fairings::RateLimiter::new(10000, 10000);
    client_with_rate_limiter(rate_limiter).await
}

pub(crate) async fn client_with_token_url(url: &str) -> Client {
    let rate_limiter = crate::fairings::RateLimiter::new(10000, 10000);
    client_with_rate_limiter_and_token_url(rate_limiter, url).await
}

pub(crate) async fn client_with_rate_limiter(rate_limiter: crate::fairings::RateLimiter) -> Client {
    let url = mock_token_list_url().await;
    client_with_rate_limiter_and_token_url(rate_limiter, &url).await
}

async fn client_with_rate_limiter_and_token_url(
    rate_limiter: crate::fairings::RateLimiter,
    url: &str,
) -> Client {
    let id = uuid::Uuid::new_v4();
    let pool = crate::db::init(&format!("sqlite:file:{id}?mode=memory&cache=shared"))
        .await
        .expect("database init");
    let token_list_url = crate::routes::tokens::TokenListUrl(url.to_string());
    Client::tracked(
        crate::rocket(pool, rate_limiter, token_list_url).expect("valid rocket instance"),
    )
    .await
    .expect("valid client")
}

async fn mock_token_list_url() -> String {
    const BODY: &str = r#"{"tokens":[{"chainId":8453,"address":"0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913","name":"USD Coin","symbol":"USDC","decimals":6}]}"#;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock token server");
    let addr = listener.local_addr().expect("mock token server address");
    let response = format!(
        "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{BODY}",
        BODY.len()
    );

    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                break;
            };

            let response = response.clone();
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                let _ = tokio::io::AsyncReadExt::read(&mut socket, &mut buf).await;
                let _ = tokio::io::AsyncWriteExt::write_all(&mut socket, response.as_bytes()).await;
            });
        }
    });

    format!("http://{addr}")
}

pub(crate) async fn seed_api_key(client: &Client) -> (String, String) {
    let key_id = uuid::Uuid::new_v4().to_string();
    let secret = uuid::Uuid::new_v4().to_string();
    let hash = crate::auth::hash_secret(&secret).expect("hash secret");

    let pool = client
        .rocket()
        .state::<crate::db::DbPool>()
        .expect("pool in state");
    sqlx::query("INSERT INTO api_keys (key_id, secret_hash, label, owner) VALUES (?, ?, ?, ?)")
        .bind(&key_id)
        .bind(&hash)
        .bind("test-key")
        .bind("test-owner")
        .execute(pool)
        .await
        .expect("insert api key");

    (key_id, secret)
}

pub(crate) fn basic_auth_header(key_id: &str, secret: &str) -> String {
    let encoded = base64::engine::general_purpose::STANDARD.encode(format!("{key_id}:{secret}"));
    format!("Basic {encoded}")
}
