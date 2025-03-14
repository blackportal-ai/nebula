//! Tracing Setup

use std::{io, path::PathBuf};

use color_eyre::eyre::Report;
use http::Request;
use tracing::{Level, Span, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{Layer as _, layer::SubscriberExt as _, util::SubscriberInitExt as _};
use uuid::Uuid;

pub struct AppDefaultValuesFromEnv {
    pub proj_name: String,
    pub data_folder: PathBuf,
    pub config_folder: PathBuf,
    pub log_env: String,
    pub log_file: String,
    pub crate_name: String,
}

/// initializes a file and optionally a console subscriber for tracing
///
/// console_lvl: Option contains the level that shall be logged onto the console
/// env_vars: A datastructure with variables catched at binary-crate level from the environment
///
/// # Safety
///
/// Besides from windows this function has to be called as long as the application is
/// still single-threaded. See [safety section](https://doc.rust-lang.org/std/env/fn.set_var.html#safety) of
/// [std::env::set_var].
pub unsafe fn initialize_logging(
    console_lvl: Option<LevelFilter>,
    env_vars: AppDefaultValuesFromEnv,
) -> Result<(), Report> {
    let directory = env_vars.data_folder;
    std::fs::create_dir_all(directory.clone())?;
    let log_path = directory.join(env_vars.log_file);
    let log_file = std::fs::File::create(log_path)?;

    unsafe {
        std::env::set_var(
            "RUST_LOG",
            std::env::var("RUST_LOG")
                .or_else(|_| std::env::var(env_vars.log_env))
                .unwrap_or_else(|_| format!("{}=info", env_vars.crate_name)),
        );
    }

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(true)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    if let Some(lvl) = console_lvl {
        let console_subscriber = tracing_subscriber::fmt::layer()
            .with_writer(io::stdout)
            .with_target(false)
            .with_ansi(true)
            .without_time()
            .with_filter(lvl);

        tracing_subscriber::registry()
            .with(file_subscriber)
            .with(console_subscriber)
            .with(ErrorLayer::default())
            .init();
    } else {
        tracing_subscriber::registry().with(file_subscriber).with(ErrorLayer::default()).init();
    }
    Ok(())
}

pub fn tracing_span_for_request(request: &Request<()>) -> Span {
    let tracing_span = tracing::span!(
        Level::INFO,
        "received request",
        req_id = Uuid::new_v4().to_string(),
        method = request.method().to_string(),
        uri = request.uri().to_string(),
        error = tracing::field::Empty,
        error_chain = tracing::field::Empty,
    );
    tracing_span
}
