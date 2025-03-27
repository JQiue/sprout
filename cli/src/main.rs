mod assets;
mod helper;
mod rpc;

use core::panic;
use std::{
  fs,
  io::{BufRead, BufReader},
  path::Path,
  process::{Command, Stdio},
  str::{self},
  thread,
  time::Duration,
};

use clap::{arg, Parser, ValueEnum};
use console::{style, Emoji};
use helper::{audit_directory, load_keywords_from_embedded, tar_directory};
use indicatif::{MultiProgress, ProgressBar, ProgressIterator, ProgressStyle};
use rpc::Rpc;

#[derive(Parser)]
#[command(name = "cli")]
#[command(author = "JQiue")]
#[command(version = "0.1.0")]
#[command(about = "a tutorial of crate clap", long_about = None)]
struct Cli {
  mode: Mode,
  #[arg(long)]
  target: Option<String>,
  #[arg(long, default_value_t = false)]
  skip_build: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
  Login,
  Deploy,
}

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");

#[derive(Debug)]
enum ProjectType {
  Vuepress,
  Custom,
  Unknown,
}

fn get_project_type() -> ProjectType {
  let cli = Cli::parse();
  if cli.target.is_some() {
    return ProjectType::Custom;
  }
  if let Ok(content) = fs::read("./package.json") {
    if let Ok(content) = String::from_utf8(content) {
      if content.contains("vuepress") {
        return ProjectType::Vuepress;
      }
    }
  }
  ProjectType::Unknown
}

fn build_project(project_type: ProjectType) -> String {
  let cli = Cli::parse();
  match project_type {
    ProjectType::Vuepress => {
      if cli.skip_build {
        return "./docs/.vuepress/dist".to_string();
      }
      let mut child = Command::new("npm.cmd")
        .arg("run")
        .arg("build")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Build failed");
      let stdout = child.stdout.take().expect("Failed to capture stdout");
      let stderr = child.stderr.take().expect("Failed to capture stderr");
      let stdout_reader = BufReader::new(stdout);
      for line in stdout_reader.lines() {
        match line {
          Ok(line) => println!("{}", line),
          Err(e) => println!("Error reading stdout: {}", e),
        }
      }
      let stderr_reader = BufReader::new(stderr);
      for line in stderr_reader.lines() {
        match line {
          Ok(line) => println!("{}", line),
          Err(e) => println!("Error reading stderr: {}", e),
        }
      }
      let status = child.wait().expect("Failed to wait for build process");
      if status.success() {
        println!("Build succeeded!");
      } else {
        println!("Build failed with status: {}", status);
      }
      "./docs/.vuepress/dist".to_string()
    }
    ProjectType::Custom => {
      let path = cli.target.clone().unwrap() + "/index.html";
      if !Path::new(&path).exists() {
        panic!("not found index.html")
      }
      return cli.target.unwrap();
    }
    ProjectType::Unknown => panic!("unknown project type"),
  }
}

fn audit_project(path: String) {
  let path = Path::new(&path);
  if !path.exists() {
    panic!("Path does not exist: {:?}", path);
  }
  let keywords = load_keywords_from_embedded(&vec![
    "./涉枪涉爆违法信息关键词.txt",
    "./色情类.txt",
    "./政治类.txt",
  ]);
  let necative_keywords = load_keywords_from_embedded(&vec!["./否定关键词.txt"]);
  let res = audit_directory(path, &keywords, &necative_keywords).expect("Cannot audit directory");
  if res.len() != 0 {
    println!("{:?}", res);
    panic!("Audit failed")
  }
}

fn get_site_id() {}

fn is_login() -> bool {
  false
}

async fn preview_project() {
  let rpc = Rpc::new("http://127.0.0.1:3000".to_string());
  let token = rpc.get_casual_token().await;
  println!("{token}");
  let deploy_data = rpc.deploy(token).await;
  println!("{:?}", deploy_data);
}

async fn deploy_project(path: String) -> String {
  tar_directory(path, "./dist.tar".to_string());
  if is_login() {
    let site_id = get_site_id();
    "root.is.me".to_string()
  } else {
    preview_project().await;
    "root.is.me".to_string()
  }
}

async fn deploy() {
  let pb = ProgressBar::new_spinner();
  pb.enable_steady_tick(Duration::from_millis(120));
  pb.set_style(
    ProgressStyle::with_template("{spinner:.blue} {msg}")
      .unwrap()
      .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸", "🎉"]),
  );
  pb.set_message(format!(
    "{} Get project type...",
    style("[1/4]").bold().dim(),
  ));
  let project_type = get_project_type();
  println!("{:?}", project_type);
  pb.set_message(format!(
    "{} {}Build project...",
    style("[2/4]").bold().dim(),
    LOOKING_GLASS
  ));
  let path = build_project(project_type);
  println!("{:?}", path);
  pb.set_message(format!(
    "{} {}Content review...",
    style("[3/4]").bold().dim(),
    LOOKING_GLASS
  ));
  audit_project(path.clone());
  println!("Review success");
  pb.set_message(format!(
    "{} {}Deploy site...",
    style("[4/4]").bold().dim(),
    LOOKING_GLASS
  ));
  let domian = deploy_project(path).await;
  println!("Deploy success!");
  if is_login() {
    pb.finish_with_message(format!(
      "Bind domian: {}, Please use cname to bind the domian",
      style(domian).cyan(),
    ));
  } else {
    pb.finish_with_message(format!(
      "Preview site: {}, Please use click to preview the site",
      style(domian).cyan(),
    ));
  }
}

#[tokio::main]
async fn main() {
  let cli = Cli::parse();
  match cli.mode {
    Mode::Login => println!("login"),
    Mode::Deploy => deploy().await,
  };
}
