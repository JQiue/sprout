use std::{
  fs,
  io::{BufRead, BufReader},
  panic,
  path::Path,
  process::{Command, Stdio},
};

use clap::Parser;
use console::style;
use tracing::{debug, trace};

use crate::{
  Cli, MASTER_URL,
  error::Error,
  helper::{
    Process, audit_directory, get_cli_config, get_project_config, load_keywords_from_embedded,
    set_project_config, tar_directory,
  },
};

// static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");

fn audit_project(path: String) {
  let path = Path::new(&path);
  if !path.exists() {
    panic!("Path does not exist: {:?}", path);
  }
  let keywords =
    load_keywords_from_embedded(&["涉枪涉爆违法信息关键词.txt", "色情类.txt", "政治类.txt"]);
  let necative_keywords = load_keywords_from_embedded(&["否定关键词.txt"]);
  let res = audit_directory(path, &keywords, &necative_keywords).expect("Cannot audit directory");
  if !res.is_empty() {
    debug!("{:?}", res);
    panic!("Audit failed")
  }
}

#[derive(Debug)]
pub enum ProjectType {
  Vuepress,
  Custom,
  Unknown,
}

pub fn get_project_type(target: Option<String>) -> ProjectType {
  if target.is_some() {
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

pub fn build_project(
  project_type: ProjectType,
  target: Option<String>,
  skip_build: bool,
) -> String {
  match project_type {
    ProjectType::Vuepress => {
      if skip_build {
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
      let path = target.clone().unwrap() + "/index.html";
      if !Path::new(&path).exists() {
        panic!("Not found index.html")
      }
      target.unwrap()
    }
    ProjectType::Unknown => panic!("Unknown project type"),
  }
}

async fn deploy_project(path: String) -> Result<(String, Option<String>), Error> {
  let cli = Cli::parse();
  let bind_domain = if let Some(domain) = cli.bind_domain {
    let mut pc = get_project_config();
    pc.bind_domain = Some(domain.clone());
    set_project_config(pc);
    Some(domain)
  } else {
    get_project_config().bind_domain
  };
  let master_rpc = rpc::MasterRpc::new(MASTER_URL.to_string())?;
  let agent_rpc = rpc::AgentRpc::new()?;
  if let Some(token) = get_cli_config().token {
    if let Some(site_id) = get_project_config().site_id {
      let path = tar_directory(path.clone(), &site_id);
      let deploy_data = master_rpc
        .create_deployment(site_id.clone(), &token)
        .await?;
      agent_rpc
        .upload_file(
          &deploy_data.deploy_url,
          deploy_data.deploy_token,
          deploy_data.deployment_id,
          path,
        )
        .await?;
      let assign_task_data = master_rpc
        .publish_site(&token, site_id, deploy_data.deployment_id, bind_domain)
        .await?;
      Ok((
        assign_task_data.preview_url,
        get_project_config().bind_domain,
      ))
    } else {
      let create_site_data = master_rpc.create_site(&token).await?;
      let mut project_config = get_project_config();
      project_config.site_id = Some(create_site_data.site_id.clone());
      set_project_config(project_config);
      let path = tar_directory(path.clone(), &create_site_data.site_id);
      let deploy_data = master_rpc
        .create_deployment(create_site_data.site_id.clone(), &token)
        .await?;
      agent_rpc
        .upload_file(
          &deploy_data.deploy_url,
          deploy_data.deploy_token,
          deploy_data.deployment_id,
          path,
        )
        .await?;
      let assign_task_data = master_rpc
        .publish_site(
          &token,
          create_site_data.site_id,
          deploy_data.deployment_id,
          bind_domain,
        )
        .await?;
      Ok((
        assign_task_data.preview_url,
        get_project_config().bind_domain,
      ))
    }
  } else {
    let get_casual_token_data = master_rpc.get_casual_token().await?;
    let create_site_data = master_rpc.create_site(&get_casual_token_data.token).await?;
    let mut project_config = get_project_config();
    project_config.site_id = Some(create_site_data.site_id.clone());
    set_project_config(project_config);
    let path = tar_directory(path.clone(), &create_site_data.site_id);
    trace!("{:?}", create_site_data);
    let deploy_data = master_rpc
      .create_deployment(
        create_site_data.site_id.clone(),
        &get_casual_token_data.token,
      )
      .await?;
    let agent_rpc = rpc::AgentRpc::new()?;
    let _ = agent_rpc
      .upload_file(
        &deploy_data.deploy_url,
        deploy_data.deploy_token,
        deploy_data.deployment_id,
        path,
      )
      .await;
    let assign_task_data = master_rpc
      .publish_site(
        &get_casual_token_data.token,
        create_site_data.site_id,
        deploy_data.deployment_id,
        None,
      )
      .await?;
    Ok((assign_task_data.preview_url, None))
  }
}

pub async fn deploy(target: Option<String>, skip_build: bool) -> Result<(), Error> {
  let pb1 = Process::new(&format!(
    "{} Get project type...",
    style("[1/4]").bold().dim()
  ));
  let project_type = get_project_type(target.clone());
  pb1.finish(None);

  let pb2 = Process::new(&format!("{} Build project...", style("[2/4]").bold().dim()));
  let path = build_project(project_type, target, skip_build);
  pb2.finish(None);

  let pb3 = Process::new(&format!(
    "{} Content review...",
    style("[3/4]").bold().dim()
  ));
  audit_project(path.clone());
  pb3.finish(None);

  let pb4 = Process::new(&format!("{} Deploy site...", style("[4/4]").bold().dim()));
  let (preview_url, bind_url) = deploy_project(path).await?;
  pb4.finish(None);

  let finish_msg = if let Some(bind_url) = bind_url {
    format!(
      "Your url: {}, Preview url: {}",
      style("https://".to_string() + &bind_url).green(),
      style(preview_url).green()
    )
  } else {
    format!("Preview url: {}", style(preview_url).cyan())
  };
  println!("{finish_msg}",);
  Ok(())
}
