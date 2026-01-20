use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, Registry, fmt};

#[derive(Debug)]
enum LogStyle {
    Pretty,
    Json,
}

impl LogStyle {
    fn from_env_or_build() -> Self {
        if let Ok(val) = std::env::var("LOG_STYLE") {
            match val.to_lowercase().as_str() {
                "pretty" | "human" => return LogStyle::Pretty,
                "json" | "bunyan" => return LogStyle::Json,
                _ => {}
            }
        }

        if cfg!(debug_assertions) {
            LogStyle::Pretty
        } else {
            LogStyle::Json
        }
    }
}

pub fn init(service_name: &str) -> WorkerGuard {
    // Non-blocking stdout writer (Cloud Run safe)
    let (nb_writer, guard): (NonBlocking, WorkerGuard) =
        tracing_appender::non_blocking(std::io::stdout());

    // -----------------------------
    // Pretty (dev / local)
    // -----------------------------
    let pretty_layer = fmt::layer()
        .compact()
        .with_writer(nb_writer.clone())
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_ansi(true)
        .with_level(true);

    // -----------------------------
    // JSON / Bunyan (prod)
    // -----------------------------
    let bunyan_layer = BunyanFormattingLayer::new(service_name.to_string(), nb_writer.clone());

    // -----------------------------
    // Env filter (IMPORTANT)
    // -----------------------------
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let registry = Registry::default().with(env_filter).with(JsonStorageLayer);

    match LogStyle::from_env_or_build() {
        LogStyle::Pretty => {
            registry
                .with(pretty_layer)
                .try_init()
                .expect("Failed to init pretty tracing subscriber");
        }
        LogStyle::Json => {
            registry
                .with(bunyan_layer)
                .try_init()
                .expect("Failed to init bunyan tracing subscriber");
        }
    }

    guard
}
