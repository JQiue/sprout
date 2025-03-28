use std::{
  env::temp_dir,
  fs::{self, File},
  io::{self, BufRead, BufReader},
  path::{Path, PathBuf},
  str::from_utf8,
};

use aho_corasick::AhoCorasick;
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

/// Audits a directory for files containing specified keywords.
///
/// This function recursively traverses the given directory and its subdirectories,
/// checking each file for the presence of specified keywords. It collects information
/// about files that contain matches.
///
/// # Parameters
///
/// * `dir` - A reference to a `Path` representing the directory to audit.
/// * `keywords` - A slice of `String`s containing the keywords to search for in each file.
///
/// # Returns
///
/// * `io::Result<Vec<FileInfo>>` - A Result containing either:
///   - `Ok(Vec<FileInfo>)`: A vector of `FileInfo` structs, each representing a file
///     that contains at least one of the specified keywords. The `FileInfo` includes
///     the file path and the line numbers where matches were found.
///   - `Err(std::io::Error)`: An I/O error if directory traversal or file reading fails.
///
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

/// Loads keywords from multiple files into a vector of strings.
///
/// This function reads each file specified in the input array, extracts non-empty lines,
/// and collects them into a single vector of keywords.
///
/// # Parameters
///
/// * `file_paths` - A slice of string slices, where each string is a path to a file containing keywords.
///
/// # Returns
///
/// * `Result<Vec<String>, std::io::Error>` - A Result containing either:
///   - `Ok(Vec<String>)`: A vector of strings, where each string is a non-empty line (trimmed) from the input files.
///   - `Err(std::io::Error)`: An I/O error if file reading fails.
///
pub fn load_keywords_from_files(file_paths: &[&str]) -> Result<Vec<String>, std::io::Error> {
  let mut keywords = Vec::new();
  for &file_path_str in file_paths {
    let file_path = Path::new(file_path_str);
    let reader = BufReader::new(File::open(file_path)?);
    for line_result in reader.lines() {
      let line = line_result?;
      if !line.trim().is_empty() {
        keywords.push(line.trim().to_string());
      }
    }
  }
  Ok(keywords)
}

pub fn tar_directory(source: String, filename: String) -> PathBuf {
  let temp = temp_dir().join(format!("{filename}.tar"));
  println!(">>> tar dist to {:?}", temp.clone());
  let mut builder = Builder::new(File::create(temp.clone()).unwrap());
  builder.append_dir_all(filename, source).unwrap();
  builder.finish().unwrap();
  temp
}

#[cfg(test)]
mod test {
  use std::time::Instant;

  use super::*;

  #[test]
  pub fn test_audit_directory() {
    let keywords = load_keywords_from_files(&vec![
      "./涉枪涉爆违法信息关键词.txt",
      "./色情类.txt",
      "./政治类.txt",
    ])
    .unwrap();
    let necative_keywords = load_keywords_from_files(&vec!["./否定关键词.txt"]).unwrap();
    println!("keywords len: {}", keywords.len());
    let start = Instant::now();
    let res = audit_directory(Path::new("./dist"), &keywords, &necative_keywords).unwrap();
    let duration = start.elapsed();
    println!("duration: {}s", duration.as_secs_f32());
    println!("{:#?}, len: {}", res, res.len());
  }

  #[test]
  pub fn test_tar_directory() {
    tar_directory("./".to_owned(), "cli.tar".to_owned());
  }
}
