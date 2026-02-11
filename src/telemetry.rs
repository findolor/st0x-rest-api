use std::sync::Once;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static TELEMETRY_INIT: Once = Once::new();

pub fn init() {
    TELEMETRY_INIT.call_once(|| {
        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|e| {
            eprintln!("invalid RUST_LOG filter, using default: {e}");
            EnvFilter::new("st0x_rest_api=info,rocket=warn,warn")
        });

        let init_result = tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().json())
            .try_init();

        if let Err(err) = init_result {
            eprintln!("failed to initialize tracing subscriber: {err}");
            std::process::exit(1);
        }
    });
}
