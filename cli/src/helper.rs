use std::{
  env::temp_dir,
  fs::{self, File},
  io::{self, BufRead, BufReader},
  path::{Path, PathBuf},
  str::from_utf8,
};

use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};
use tar::Builder;

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
  path: PathBuf,
  info: Vec<(String, usize, usize)>,
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
          // println!("skip file: {:?}", path);
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

pub fn load_keywords_from_embedded(file_paths: &[&str]) -> Vec<std::string::String> {
  let mut keywords = Vec::new();
  for file_path_str in file_paths {
    let e = Asset::get(file_path_str).unwrap();
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
  println!(">>> tar dist to {:?}", temp.clone());
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
    let config = serde_json::from_str::<CliConfig>(&config_str).unwrap();
    return config;
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
}

pub fn get_project_config() -> ProjectConfig {
  if !fs::exists("./.pupup/config.json").unwrap() {
    fs::create_dir_all(".pupup").unwrap();
    fs::write("./.pupup/config.json", "{}").unwrap();
  }

  let config_str = fs::read_to_string("./.pupup/config.json").unwrap();
  let config = serde_json::from_str::<ProjectConfig>(&config_str).unwrap();
  config
}

pub fn set_project_config(config: ProjectConfig) {
  if !fs::exists("./.pupup/config.json").unwrap() {
    fs::create_dir_all(".pupup").unwrap();
    fs::write("./.pupup/config.json", "{}").unwrap();
  }
  fs::write(
    "./.pupup/config.json",
    serde_json::to_string(&config).unwrap(),
  )
  .unwrap();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn test_tar_directory() {
    tar_directory("./".to_owned(), "cli.tar");
  }
}
