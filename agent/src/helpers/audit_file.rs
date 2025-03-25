use std::{
  fs::{self, File},
  io::{self, BufRead, BufReader},
  path::{Path, PathBuf},
};

use aho_corasick::AhoCorasick;

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
  line_numbers: Vec<usize>,
}

pub fn audit_file(file_path: &Path, keywords: &[String]) -> Result<Vec<usize>, std::io::Error> {
  let ac = AhoCorasick::new(keywords).unwrap();
  // 打开文件
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  // 存储匹配结果
  let mut matches = Vec::new();
  // 逐行读取文件并进行审核
  for (index, line) in reader.lines().enumerate() {
    let line = line?;
    if ac.find(&line).is_some() {
      matches.push(index + 1);
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
pub fn audit_directory(dir: &Path, keywords: &[String]) -> io::Result<Vec<FileInfo>> {
  let mut results = Vec::new();
  let keywords = keywords.to_vec(); // 克隆关键词列表以满足闭包的生命周期要求
  let mut audit_callback = |path: &Path| -> io::Result<()> {
    if let Ok(matches) = audit_file(path, &keywords) {
      if !matches.is_empty() {
        // results.push((path.to_path_buf(), matches));
        results.push(FileInfo {
          path: path.to_path_buf(),
          line_numbers: matches,
        });
      }
    }
    Ok(())
  };

  visit_dirs(dir, &mut audit_callback)?;
  Ok(results)
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
    println!("{:?}", file_path);
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

#[cfg(test)]
mod test {
  use std::time::Instant;

  use super::*;

  #[test]
  pub fn test() {
    let keywords = load_keywords_from_files(&vec![
      "./涉枪涉爆违法信息关键词.txt",
      "./色情类.txt",
      "./政治类.txt",
    ])
    .unwrap();
    println!("keywords len: {}", keywords.len());
    let start = Instant::now();
    let res = audit_directory(Path::new("./dist"), &keywords).unwrap();
    let duration = start.elapsed();
    println!("duration：{}s", duration.as_secs_f32());
    println!("{:#?}, len: {}", res, res.len());
  }
}
