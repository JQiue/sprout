mod assets;
mod commands;
mod helper;

use clap::{Parser, ValueEnum};
use commands::{deploy::deploy, login::login};
use log::LevelFilter;

#[derive(Parser)]
#[command(name = "cli")]
#[command(author = "JQiue")]
#[command(version = "0.1.0")]
#[command(about = "a tutorial of crate clap", long_about = None)]
pub struct Cli {
  command: Commands,
  #[arg(long, help = "Specify deployment directory")]
  target: Option<String>,
  #[arg(long, default_value_t = false, help = "Skip the build step")]
  skip_build: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Commands {
  Login,
  Deploy,
}

static MASTER_URL: &str = "http://127.0.0.1:3000";

#[tokio::main]
async fn main() {
  env_logger::builder()
    .filter_level(LevelFilter::Off)
    .parse_default_env()
    .init();
  match Cli::parse().command {
    Commands::Login => login().await,
    Commands::Deploy => deploy().await,
  };
}
