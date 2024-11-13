mod app;
mod components;
mod config;
mod helpers;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  std::env::set_var("RUST_LOG", "info");
  tracing_subscriber::fmt::init();
  app::start().await
}
