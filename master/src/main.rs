mod app;
mod cloudflare;
mod components;
mod config;
mod entities;
mod middleware;
mod response;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();
  app::start().await
}
