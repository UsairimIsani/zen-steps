use axum::extract::FromRef;
use tracing_subscriber::{EnvFilter, prelude::*};
use crate::error::Error;

use crate::config::EnvConfig;


#[derive(FromRef, Clone)]
pub struct AppState {
    pub env:EnvConfig}

impl AppState{
    pub fn new(env: EnvConfig)-> Self{

        Self{env}
    }
}
pub fn setup() -> Result<(), Error> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let tracing_fmt_layer = tracing_subscriber::fmt::layer();

    let env_filter = EnvFilter::from_default_env();

    tracing_subscriber::registry()
        .with(tracing_fmt_layer)
        .with(env_filter)
        .init();

    Ok(())
}