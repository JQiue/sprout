use std::{
  fs::{self, File},
  io::{self, BufRead, BufReader},
  path::{Path, PathBuf},
  time::Instant,
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
  // 创建正则表达式模式
  // let pattern = keywords.join("|");
  // let re = Regex::new(&pattern).unwrap();
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
    // if re.is_match(&line) {
    //   matches.push(index + 1);
    //   //   matches.push((index + 1, line));
    // }
  }

  Ok(matches)
}

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

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
  let file1 = File::open(Path::new("./涉枪涉爆违法信息关键词.txt"))?;
  let reader1 = BufReader::new(file1);
  let mut keywords = vec![];
  for (_index, line) in reader1.lines().enumerate() {
    let line = line?;
    keywords.push(line.clone());
  }
  let file2 = File::open(Path::new("./色情类.txt"))?;
  let reader2 = BufReader::new(file2);
  for (_index, line) in reader2.lines().enumerate() {
    let line = line?;
    keywords.push(line.clone());
  }
  let file3 = File::open(Path::new("./政治类.txt"))?;
  let reader3 = BufReader::new(file3);
  for (_index, line) in reader3.lines().enumerate() {
    let line = line?;
    keywords.push(line.clone());
  }
  println!("keywords len: {}", keywords.len());
  let start = Instant::now();
  let res = audit_directory(Path::new("./dist"), &keywords).unwrap();
  let duration = start.elapsed();
  println!("duration：{}s", duration.as_secs_f32());
  println!("{:#?}, len: {}", res, res.len());

  Ok(())
}
