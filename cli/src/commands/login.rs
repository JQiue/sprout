use console::Color;
use tracing::debug;

use crate::{
  MASTER_URL,
  error::Error,
  helper::{console_print, get_cli_config, prompt_email, prompt_password, set_cli_config},
};

pub async fn login() -> Result<(), Error> {
  debug!(">>> login");
  console_print("Welcome to the CLI! ", None, true, false);
  console_print(
    "Please enter your login info:",
    Some(Color::Cyan),
    true,
    true,
  );
  let email = prompt_email();
  let password = prompt_password(false);
  let rpc = rpc::MasterRpc::new(MASTER_URL.to_string())?;
  let login_data = rpc.login(email, password).await?;
  let mut cli_config = get_cli_config();
  cli_config.token = Some(login_data.token);
  set_cli_config(cli_config);
  console_print(
    "Login successful! You can now use the CLI.",
    Some(Color::Green),
    false,
    false,
  );
  Ok(())
}
