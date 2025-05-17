use console::Color;
use tracing::debug;

use crate::{
  MASTER_URL,
  error::Error,
  helper::{console_print, prompt_email, prompt_password, prompt_user},
};

pub async fn signup() -> Result<(), Error> {
  debug!(">>> singnup");
  console_print("Welcome to the CLI! ", None, true, false);
  console_print(
    "Please enter your sign up info:",
    Some(Color::Cyan),
    true,
    true,
  );
  let nickname: String = prompt_user("Nickname");
  let email: String = prompt_email();
  let password = prompt_password(true);
  let rpc = rpc::MasterRpc::new(MASTER_URL.to_string())?;
  rpc.signup(nickname, email, password).await?;
  console_print(
    "Sign up successful! You can now use `pupup login` to login.",
    Some(Color::Green),
    false,
    true,
  );
  Ok(())
}

#[cfg(test)]
mod test {
  use console::{Key, Term, style};
  use dialoguer::{
    Completion, Confirm, Input, MultiSelect, Password, Select, Sort, theme::ColorfulTheme,
  };

  #[test]
  fn test_console() {
    // println!(
    //   "This is red on black: {:010x}",
    //   style(42).red().on_black().bold()
    // );
    // println!("This is reversed: [{}]", style("whatever").reverse());
    // println!("This is cyan: {}", style("whatever").cyan());
    // eprintln!(
    //   "This is black bright: {}",
    //   style("whatever").for_stderr().bright().black()
    // );
    // for i in 0..=255 {
    //   print!("{:03} ", style(i).color256(i));
    //   if i % 16 == 15 {
    //     println!();
    //   }
    // }

    // for i in 0..=255 {
    //   print!("{:03} ", style(i).black().on_color256(i));
    //   if i % 16 == 15 {
    //     println!();
    //   }
    // }

    // let term = Term::stdout();
    // let (height, width) = term.size();
    // for x in 0..width {
    //   for y in 0..height {
    //     term.move_cursor_to(x as usize, y as usize).unwrap();
    //     let text = if (x + y) % 2 == 0 {
    //       format!("{}", style(x % 10).black().on_red())
    //     } else {
    //       format!("{}", style(x % 10).red().on_black())
    //     };

    //     term.write_str(&text).unwrap();
    //     thread::sleep(Duration::from_millis(100));
    //   }
    // }

    // let raw = std::env::args_os().any(|arg| arg == "-r" || arg == "--raw");
    // let term = Term::stdout();
    // term.write_line("Press any key. Esc to exit").unwrap();
    // loop {
    //   let key = if raw {
    //     term.read_key_raw()
    //   } else {
    //     term.read_key()
    //   }
    //   .unwrap();
    //   term.write_line(&format!("You pressed {:?}", key)).unwrap();
    //   if key == Key::Escape {
    //     break;
    //   }
    // }
  }

  #[test]
  fn test_deploy() {
    let term = Term::buffered_stderr();
    let theme = ColorfulTheme::default();
    println!("All the following controls are run in a buffered terminal");
    Confirm::with_theme(&theme)
      .with_prompt("Do you want to continue?")
      .interact_on(&term)
      .unwrap();

    let _: String = Input::with_theme(&theme)
      .with_prompt("Your name")
      .interact_on(&term)
      .unwrap();

    let items = &[
      "Ice Cream",
      "Vanilla Cupcake",
      "Chocolate Muffin",
      "A Pile of sweet, sweet mustard",
    ];

    Select::with_theme(&theme)
      .with_prompt("Pick an item")
      .items(items)
      .interact_on(&term)
      .unwrap();

    MultiSelect::with_theme(&theme)
      .with_prompt("Pick some items")
      .items(items)
      .interact_on(&term)
      .unwrap();

    Sort::with_theme(&theme)
      .with_prompt("Order these items")
      .items(items)
      .interact_on(&term)
      .unwrap();
  }

  #[test]
  fn test_one() {
    struct MyCompletion {
      options: Vec<String>,
    }

    impl Default for MyCompletion {
      fn default() -> Self {
        MyCompletion {
          options: vec![
            "orange".to_string(),
            "apple".to_string(),
            "banana".to_string(),
          ],
        }
      }
    }

    impl Completion for MyCompletion {
      /// Simple completion implementation based on substring
      fn get(&self, input: &str) -> Option<String> {
        let matches = self
          .options
          .iter()
          .filter(|option| option.starts_with(input))
          .collect::<Vec<_>>();

        if matches.len() == 1 {
          Some(matches[0].to_string())
        } else {
          None
        }
      }
    }

    let completion = MyCompletion::default();

    Input::<String>::with_theme(&ColorfulTheme::default())
      .with_prompt("dialoguer")
      .completion_with(&completion)
      .interact_text()
      .unwrap();
  }
}
