use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod components;
mod config;
mod error;
mod helpers;
mod response;
mod rpc;

#[actix_web::main]
async fn main() -> Result<(), error::AppError> {
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_timer(fmt::time::LocalTime::rfc_3339()))
    .init();
  tracing_subscriber::fmt::init();
  app::start().await
}
