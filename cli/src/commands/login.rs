use std::io;

use console::{Style, Term};
use dialoguer::Password;
use log::trace;

use crate::{
  helper::{get_cli_config, set_cli_config},
  rpc::MasterRpc,
};

pub async fn login() {
  trace!(">>> login");
  let term = Term::stdout();
  let bold_style = Style::new().bold();
  let green_style = Style::new().green();
  term
    .write_line(&format!(
      "{} {}",
      bold_style.apply_to("Welcome to the CLI!"),
      green_style.apply_to("Please enter your login info:")
    ))
    .unwrap();
  term
    .write_str(&format!("{} ", bold_style.apply_to("Username:")))
    .unwrap();
  let mut username = String::new();
  io::stdin()
    .read_line(&mut username)
    .expect("Failed to read line");
  username = username.trim().to_string();
  term
    .write_str(&format!("{} ", bold_style.apply_to("Password:")))
    .unwrap();
  let password = Password::new()
    .with_prompt(bold_style.apply_to("Password:").to_string())
    .interact()
    .unwrap();
  trace!(">>> username: {username}, password: {password}");
  let rpc = MasterRpc::new();
  let token = rpc.login(username, password).await;
  trace!(">>> token: {token}");
  let mut cli_config = get_cli_config();
  cli_config.token = Some(token);
  set_cli_config(cli_config);
}
