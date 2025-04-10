use std::{
  fs,
  io::{BufRead, BufReader},
  panic,
  path::Path,
  process::{Command, Stdio},
  time::Duration,
};

use crate::{
  Cli, MASTER_URL,
  helper::{
    audit_directory, get_cli_config, get_project_config, load_keywords_from_embedded,
    set_project_config, tar_directory,
  },
};
use clap::Parser;
use console::{Emoji, style};
use indicatif::{ProgressBar, ProgressStyle};
use log::trace;

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");

fn is_login() -> bool {
  let cli_config = get_cli_config();
  cli_config.token.is_some()
}

fn audit_project(path: String) {
  let path = Path::new(&path);
  if !path.exists() {
    panic!("Path does not exist: {:?}", path);
  }
  let keywords = load_keywords_from_embedded(&[
    "./涉枪涉爆违法信息关键词.txt",
    "./色情类.txt",
    "./政治类.txt",
  ]);
  let necative_keywords = load_keywords_from_embedded(&["./否定关键词.txt"]);
  let res = audit_directory(path, &keywords, &necative_keywords).expect("Cannot audit directory");
  if !res.is_empty() {
    trace!("{:?}", res);
    panic!("Audit failed")
  }
}

#[derive(Debug)]
pub enum ProjectType {
  Vuepress,
  Custom,
  Unknown,
}

pub fn get_project_type() -> ProjectType {
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

pub fn build_project(project_type: ProjectType) -> String {
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
      cli.target.unwrap()
    }
    ProjectType::Unknown => panic!("Unknown project type"),
  }
}

async fn deploy_project(path: String) -> String {
  let master_rpc = rpc::Master::Rpc::new(MASTER_URL.to_string());
  let agent_rpc = rpc::Agent::Rpc::new();
  if let Some(token) = get_cli_config().token {
    if let Some(site_id) = get_project_config().site_id {
      let path = tar_directory(path.clone(), &site_id);
      let deploy_data = master_rpc.create_deployment(&site_id, &token).await;
      let _ = agent_rpc
        .upload_file(
          deploy_data.deploy_url,
          deploy_data.deploy_token,
          deploy_data.deployment_id,
          path,
        )
        .await;
      let assign_task_data = master_rpc
        .publish_site(&token, &site_id, deploy_data.deployment_id)
        .await;
      assign_task_data.preview_url
    } else {
      let create_site_data = master_rpc.create_site(&token).await;
      let mut project_config = get_project_config();
      project_config.site_id = Some(create_site_data.site_id.clone());
      set_project_config(project_config);
      let path = tar_directory(path.clone(), &create_site_data.site_id);
      let deploy_data = master_rpc
        .create_deployment(&create_site_data.site_id, &token)
        .await;
      let _ = agent_rpc
        .upload_file(
          deploy_data.deploy_url,
          deploy_data.deploy_token,
          deploy_data.deployment_id,
          path,
        )
        .await;
      let assign_task_data = master_rpc
        .publish_site(&token, &create_site_data.site_id, deploy_data.deployment_id)
        .await;
      assign_task_data.preview_url
    }
  } else {
    let token = master_rpc.get_casual_token().await;
    let create_site_data = master_rpc.create_site(&token).await;
    let mut project_config = get_project_config();
    project_config.site_id = Some(create_site_data.site_id.clone());
    set_project_config(project_config);
    let path = tar_directory(path.clone(), &create_site_data.site_id);
    trace!("{:?}", create_site_data);
    let deploy_data = master_rpc
      .create_deployment(&create_site_data.site_id, &token)
      .await;
    let _ = agent_rpc
      .upload_file(
        deploy_data.deploy_url,
        deploy_data.deploy_token,
        deploy_data.deployment_id,
        path,
      )
      .await;
    let assign_task_data = master_rpc
      .publish_site(&token, &create_site_data.site_id, deploy_data.deployment_id)
      .await;
    assign_task_data.preview_url
  }
}

pub async fn deploy() {
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
      "Preview url: {}, Please use click to preview the site",
      style(domian).cyan(),
    ));
  } else {
    pb.finish_with_message(format!(
      "Preview url: {}, Please use click to preview the site",
      style(domian).cyan(),
    ));
  }
}
