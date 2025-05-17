mod assets;
mod commands;
mod error;
mod helper;

use clap::{Parser, Subcommand};
use commands::{deploy::deploy, list::list, login::login, signup::signup};
use console::Color;
use error::Error;
use helper::{console_print, print_error};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
  /// Choose the program mode run in
  #[command(subcommand)]
  command: Commands,
  #[arg(long, help = "Bind domain")]
  bind_domain: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
  /// user singn up
  Signup,
  /// user login
  Login,
  /// user deploy
  Deploy {
    #[arg(long, help = "Specify deployment directory")]
    target: Option<String>,
    #[arg(long, default_value_t = false, help = "Skip the build step")]
    skip_build: bool,
  },
  /// list all sites
  List,
}

static MASTER_URL: &str = "http://127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_timer(fmt::time::LocalTime::rfc_3339()))
    .with(EnvFilter::from_default_env())
    .init();
  match Cli::parse().command {
    Commands::Signup => signup().await?,
    Commands::Login => login().await?,
    Commands::Deploy { target, skip_build } => {
      match deploy(target, skip_build).await {
        Ok(_) => (),
        Err(err) => match err {
          Error::AuthenticationRequired => login().await?,
          Error::RpcCall => print_error("Deploy error"),
          Error::CannotConnect => print_error("Cannot connect to server"),
        },
      };
    }
    Commands::List => {
      match list().await {
        Ok(_) => (),
        Err(err) => match err {
          Error::AuthenticationRequired => login().await?,
          Error::RpcCall => (),
          Error::CannotConnect => print_error("Cannot connect to server"),
        },
      };
    }
  };
  Ok(())
}
