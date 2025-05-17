use std::{
  env::temp_dir,
  fs::{self, File},
  io::{self, BufRead, BufReader, Write},
  path::{Path, PathBuf},
  str::from_utf8,
  time::Duration,
};

use aho_corasick::AhoCorasick;
use comfy_table::{
  CellAlignment, ContentArrangement, Table,
  modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
  presets::UTF8_FULL,
};
use console::{Color, Style, Term};
use dialoguer::{Input, Password, theme::ColorfulTheme};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use tar::Builder;
use tracing::{debug, error, trace};

use crate::assets::Asset;

pub fn visit_dirs<F>(dir: &Path, callback: &mut F) -> io::Result<()>
where
  F: FnMut(&Path) -> io::Result<()>,
{
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let path = entry?.path();
      if path.is_dir() {
        visit_dirs(&path, callback)?;
      } else {
        callback(&path)?;
      }
    }
  }
  Ok(())
}

#[derive(Debug)]
pub struct FileInfo {
  pub path: PathBuf,
  pub info: Vec<(String, usize, usize)>,
}

pub fn audit_file(
  file_path: &Path,
  keywords: &[String],
  negative_keywords: &[String],
) -> Result<Vec<(String, usize, usize)>, std::io::Error> {
  let ac = AhoCorasick::new(keywords).unwrap();
  let negative_ac = AhoCorasick::new(negative_keywords).unwrap();
  let reader = BufReader::new(File::open(file_path)?);
  let mut matches = Vec::new();
  // 逐行读取文件并进行审核
  for (index, line) in reader.lines().enumerate() {
    let line = line?;
    // if ac.find(&line).is_some() {
    // }
    for mat in ac.find_iter(&line) {
      let keyword = keywords[mat.pattern()].clone();
      let start = mat.start();
      let end = mat.end();

      // // 检查周围的文本是否包含否定关键词
      // let context_start = if start > 20 { start - 20 } else { 0 };
      // let context_end = if end + 20 < line.len() {
      //   end + 20
      // } else {
      //   line.len()
      // };
      // let context = &line[context_start..context_end];

      // 确保 context_start 和 context_end 是字符的边界
      let mut context_start = start;
      let mut context_end = end;

      let mut char_count = 0;
      for (byte_index, _) in line.char_indices() {
        if byte_index <= start {
          context_start = byte_index;
        }
        if byte_index <= end {
          context_end = byte_index;
        }
        char_count += 1;
        if char_count > 20 {
          break;
        }
      }

      if start > 20 {
        let mut char_count = 0;
        for (byte_index, _) in line[..start].char_indices().rev() {
          if char_count >= 20 {
            context_start = byte_index;
            break;
          }
          char_count += 1;
        }
      } else {
        context_start = 0;
      }

      if line.len() - end > 20 {
        let mut char_count = 0;
        for (byte_index, _) in line[end..].char_indices() {
          if char_count >= 20 {
            context_end = end + byte_index;
            break;
          }
          char_count += 1;
        }
      } else {
        context_end = line.len();
      }

      let context = &line[context_start..context_end];

      if !negative_ac.is_match(context) {
        matches.push((keyword, index + 1, start + 1));
      }
    }
  }

  Ok(matches)
}

pub fn audit_directory(
  dir: &Path,
  keywords: &[String],
  negative_keywords: &[String],
) -> io::Result<Vec<FileInfo>> {
  let mut results = Vec::new();
  let keywords = keywords.to_vec();
  let mut audit_callback = |path: &Path| -> io::Result<()> {
    if path.is_dir() {
      return Ok(());
    }
    if let Some(extension) = path.extension() {
      if let Some(ext) = extension.to_str() {
        if !(ext.eq_ignore_ascii_case("html")
          || ext.eq_ignore_ascii_case("js")
          || ext.eq_ignore_ascii_case("json"))
        {
          debug!("skip file: {:?}", path);
          return Ok(());
        }
      }
    }
    if let Ok(matches) = audit_file(path, &keywords, negative_keywords) {
      if !matches.is_empty() {
        results.push(FileInfo {
          path: path.to_path_buf(),
          info: matches,
        });
      }
    }
    Ok(())
  };
  visit_dirs(dir, &mut audit_callback)?;
  Ok(results)
}

pub fn load_keywords_from_embedded(file_paths: &[&str]) -> Vec<String> {
  // for s in Asset::iter() {
  //   debug!("Asset: {:?}", s);
  // }
  let mut keywords = Vec::new();
  for file_path_str in file_paths {
    let e = if let Some(e) = Asset::get(file_path_str) {
      e
    } else {
      error!("Faild to load embedded file: {:?}", file_path_str);
      break;
    };
    let content = from_utf8(&e.data).unwrap();
    content.split("\n").for_each(|line| {
      if !line.trim().is_empty() {
        keywords.push(line.trim().to_string());
      }
    });
  }
  keywords
}

pub fn tar_directory(source: String, filename: &str) -> PathBuf {
  let temp = temp_dir().join(format!("{filename}.tar"));
  trace!(">>> tar dist to {:?}", temp.clone());
  let mut builder = Builder::new(File::create(temp.clone()).unwrap());
  builder.append_dir_all(filename, source).unwrap();
  builder.finish().unwrap();
  temp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
  pub token: Option<String>,
}

pub fn get_cli_config() -> CliConfig {
  if let Some(home) = dirs::home_dir() {
    if !fs::exists(home.join("./.pupup/config.json")).unwrap() {
      fs::create_dir_all(home.join("./.pupup")).unwrap();
      fs::write(home.join("./.pupup/config.json"), "{}").unwrap();
    }
    let config_str = fs::read_to_string(home.join("./.pupup/config.json")).unwrap();
    serde_json::from_str::<CliConfig>(&config_str).unwrap()
  } else {
    panic!("Faild to get home directory");
  }
}

pub fn set_cli_config(config: CliConfig) {
  if let Some(home) = dirs::home_dir() {
    if !fs::exists(home.join("./.pupup/config.json")).unwrap() {
      fs::create_dir(home.join("./.pupup")).unwrap();
    }
    fs::write(
      home.join("./.pupup/config.json"),
      serde_json::to_string(&config).unwrap(),
    )
    .unwrap();
  } else {
    panic!("Faild to get home directory");
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
  pub site_id: Option<String>,
  pub bind_domain: Option<String>,
}

pub fn get_project_config() -> ProjectConfig {
  if !fs::exists("./.pupup/config.json").unwrap() {
    fs::create_dir_all(".pupup").unwrap();
    fs::write("./.pupup/config.json", "{}").unwrap();
  }

  let config_str = fs::read_to_string("./.pupup/config.json").unwrap();
  serde_json::from_str::<ProjectConfig>(&config_str).unwrap_or(ProjectConfig {
    site_id: None,
    bind_domain: None,
  })
}

pub fn set_project_config(config: ProjectConfig) -> ProjectConfig {
  if !fs::exists("./.pupup/config.json").unwrap() {
    fs::create_dir_all(".pupup").unwrap();
    fs::write("./.pupup/config.json", "{}").unwrap();
  }
  fs::write(
    "./.pupup/config.json",
    serde_json::to_string(&config).unwrap(),
  )
  .unwrap();
  get_project_config()
}

pub fn draw_table(rows: Vec<Vec<String>>) {
  let mut table = Table::new();

  table
    .load_preset(UTF8_FULL)
    .apply_modifier(UTF8_SOLID_INNER_BORDERS)
    .set_content_arrangement(ContentArrangement::Dynamic)
    .add_rows(&rows);

  for (index, _) in rows.iter().enumerate() {
    let column = table.column_mut(index).unwrap();
    column.set_cell_alignment(CellAlignment::Center);
  }

  println!("{table}");
}

pub fn prompt_user(prompt: &str) -> String {
  Input::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt)
    .interact_text()
    .unwrap()
}

pub fn prompt_email() -> String {
  Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Email")
    .validate_with({
      let mut force = None;
      move |input: &String| -> Result<(), &str> {
        if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
          Ok(())
        } else {
          force = Some(input.clone());
          Err("This is not a mail address; type the same value again to force use")
        }
      }
    })
    .interact_text()
    .unwrap()
}

pub fn prompt_password(repeat: bool) -> String {
  if repeat {
    Password::with_theme(&ColorfulTheme::default())
      .with_prompt("Password")
      .with_confirmation("Repeat password", "Error: the passwords don't match.")
      .validate_with(|input: &String| -> Result<(), &str> {
        if input.len() < 8 || input.len() > 16 {
          Err("Password must be between 8 and 16 characters")
        } else {
          Ok(())
        }
      })
      .interact()
      .unwrap()
  } else {
    Password::with_theme(&ColorfulTheme::default())
      .with_prompt("Password")
      .validate_with(|input: &String| -> Result<(), &str> {
        if input.len() < 8 || input.len() > 16 {
          Err("Password must be between 8 and 16 characters")
        } else {
          Ok(())
        }
      })
      .interact()
      .unwrap()
  }
}

pub fn console_print(text: &str, color: Option<Color>, bold: bool, newline: bool) {
  let mut term = Term::stdout();
  let style = Style::new().fg(color.unwrap_or(Color::White));
  let style = if bold { style.bold() } else { style };
  term
    .write(style.apply_to(text).to_string().as_bytes())
    .unwrap();
  if newline {
    term.write_str("\n").unwrap();
  }
}

pub fn print_error(text: &str) {
  console_print(text, Some(Color::Magenta), true, true);
}

pub struct Process {
  pb: ProgressBar,
  msg: String,
}

impl Process {
  pub fn new(msg: &str) -> Self {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
      ProgressStyle::with_template("{spinner:.blue} {msg}")
        .unwrap()
        .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸", "🎉"]),
    );
    pb.set_message(msg.to_string());
    Self {
      pb,
      msg: msg.to_string(),
    }
  }

  pub fn finish(&self, msg: Option<String>) {
    if let Some(msg) = msg {
      self.pb.finish_with_message(msg);
    } else {
      self.pb.finish();
    }
  }
}

#[cfg(test)]
mod test {

  use console::Color;

  use super::*;

  #[test]
  pub fn test_tar_directory() {
    tar_directory("./".to_owned(), "cli.tar");
  }

  // #[test]
  // pub fn test_draw_table() {
  //   let rows = vec![vec!["One", "Two"], vec!["Three", "Four"]];
  //   draw_table(rows);
  // }

  #[test]
  pub fn test_console() {
    console_print("Hello", Some(Color::Cyan), true, true);
    console_print("World", None, false, true);
  }
}
