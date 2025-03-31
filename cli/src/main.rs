mod assets;
mod commands;
mod helper;
mod rpc;

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

#[tokio::main]
async fn main() {
  env_logger::builder()
    .filter_level(LevelFilter::Trace)
    .init();

  let cli = Cli::parse();
  match cli.command {
    Commands::Login => login().await,
    Commands::Deploy => deploy().await,
  };
}
