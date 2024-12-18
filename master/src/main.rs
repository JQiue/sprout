mod app;
mod components;
mod config;
mod entitys;
mod helpers;
mod middleware;
mod response;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  std::env::set_var("RUST_LOG", "debug");
  tracing_subscriber::fmt::init();
  app::start().await
}
