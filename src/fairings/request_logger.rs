use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::request::{FromRequest, Outcome};
use rocket::{Data, Request, Response};
use std::time::Instant;
use uuid::Uuid;

struct RequestMeta {
    start: Instant,
    request_id: String,
    span: tracing::Span,
}

pub struct RequestLogger;
pub struct TracingSpan(pub tracing::Span);

const REQUEST_ID_HEADER: &str = "X-Request-Id";

fn fallback_meta() -> RequestMeta {
    RequestMeta {
        start: Instant::now(),
        request_id: "unknown".to_string(),
        span: tracing::Span::none(),
    }
}

fn extract_request_id(req: &Request<'_>) -> String {
    match req.headers().get_one(REQUEST_ID_HEADER) {
        Some(value) => {
            let trimmed = value.trim();
            if !trimmed.is_empty()
                && trimmed.len() <= 128
                && trimmed.is_ascii()
                && !trimmed.chars().any(|c| c.is_control())
            {
                return trimmed.to_string();
            }
            Uuid::new_v4().to_string()
        }
        None => Uuid::new_v4().to_string(),
    }
}

pub(crate) fn request_span_for(req: &Request<'_>) -> tracing::Span {
    req.local_cache(fallback_meta).span.clone()
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TracingSpan {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(TracingSpan(request_span_for(req)))
    }
}

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let request_id = extract_request_id(req);
        let span = tracing::info_span!(
            "request",
            method = %req.method(),
            uri = %req.uri(),
            request_id = %request_id,
        );
        span.in_scope(|| tracing::info!("request started"));
        req.local_cache(|| RequestMeta {
            start: Instant::now(),
            request_id,
            span,
        });
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let meta = req.local_cache(fallback_meta);
        let duration_ms = meta.start.elapsed().as_secs_f64() * 1000.0;
        let status = res.status().code;

        meta.span.in_scope(|| {
            if status >= 500 {
                tracing::error!(status, duration_ms, "request completed");
            } else if status >= 400 {
                tracing::warn!(status, duration_ms, "request completed");
            } else {
                tracing::info!(status, duration_ms, "request completed");
            }
        });

        res.set_header(Header::new(REQUEST_ID_HEADER, meta.request_id.clone()));
    }
}
