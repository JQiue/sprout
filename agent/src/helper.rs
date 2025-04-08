use serde::{Deserialize, Serialize};
use std::{
  fs,
  io::{BufRead, BufReader},
  path::PathBuf,
  process::{Command, Stdio},
};
use tracing::{error, trace};

pub fn generate_domian(site_id: &str) -> String {
  format!("{site_id}.is.me")
}

pub fn extract_tar(filename: String, output: String) {
  let mut child = Command::new("tar")
    .arg("-xf")
    .arg(filename)
    .arg("-C")
    .arg(output)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Build failed");
  let stdout = child.stdout.take().expect("Failed to capture stdout");
  let stderr = child.stderr.take().expect("Failed to capture stderr");
  let stdout_reader = BufReader::new(stdout);
  for line in stdout_reader.lines() {
    match line {
      Ok(line) => trace!("{}", line),
      Err(e) => error!("Error reading stdout: {}", e),
    }
  }
  let stderr_reader = BufReader::new(stderr);
  for line in stderr_reader.lines() {
    match line {
      Ok(line) => trace!("{}", line),
      Err(e) => error!("Error reading stderr: {}", e),
    }
  }
  let status = child.wait().expect("Failed to wait for build process");
  if status.success() {
    println!("Build succeeded!");
  } else {
    println!("Build failed with status: {}", status);
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxConfig {
  config_path: PathBuf,
  ssl_enabled: bool,
}

impl NginxConfig {
  pub fn new(config_path: &str, ssl_enabled: bool) -> Self {
    Self {
      config_path: config_path.into(),
      ssl_enabled,
    }
  }

  pub fn generate_config(&self, domain: &str, root_path: &str) -> String {
    let mut config = String::new();
    config.push_str("server {\n");
    config.push_str("    listen 80;\n");
    config.push_str(&format!("    server_name {};\n", domain));
    config.push_str("    location / {\n");
    config.push_str("        try_files $uri $uri/ /index.html;\n");
    config.push_str(&format!("        root {};\n", root_path));
    config.push_str("        index index.html;\n");
    config.push_str("    }\n");
    config.push_str("}\n");
    config
  }

  pub fn remove_config(&self, site_id: &str) {
    let domian_config = self.config_path.join(site_id.to_string() + ".conf");
    if domian_config.exists() {
      fs::remove_file(domian_config).unwrap_or_else(|_| {
        error!("Failed to remove config file: {}.conf", site_id);
      });
    } else {
      error!("Config file does not exist: {}.conf", site_id);
    }
  }

  pub fn deploy(&self, domain: &str, root_path: &str) -> bool {
    // 生成配置文件内容
    let config_content = self.generate_config(domain, root_path);

    // 暂时简单写入，以后替换为原子写入
    if let Err(e) = fs::create_dir_all(self.config_path.as_path()) {
      tracing::error!("Failed to create config directory: {}", e);
      return false;
    }

    if !fs::write(
      self.config_path.join(format!("{}.conf", domain)),
      config_content,
    )
    .map_err(|_| {
      tracing::error!("{}", "write Nginx configuration failed");
    })
    .is_ok()
    {
      return false;
    }

    if self.ssl_enabled {
      self.apply_ssl();
    }

    // 测试配置是否正确
    if !Command::new("nginx").arg("-t").status().is_ok() {
      tracing::error!("{}", "Nginx configuration test failed");
      return false;
    }
    // 重新加载 Nginx
    if !Command::new("nginx")
      .arg("-s")
      .arg("reload")
      .status()
      .is_ok()
    {
      tracing::error!("{}", "reload Nginx failed");
      return false;
    }
    true
  }

  pub fn apply_ssl(&self) {}
}

#[cfg(test)]
mod test {
  use super::NginxConfig;

  #[test]
  pub fn test_deploy() {
    let nc = NginxConfig::new("/etc/nginx/sprout", false);
    println!("{}", nc.generate_config("jinqiu.wang", "/var/www/html"));
    nc.deploy("jinqiu.wang", "/var/www/html");
  }

  #[test]
  pub fn test_revoke() {
    let nc = NginxConfig::new("/etc/nginx/sprout", false);
    nc.remove_config("jinqiu.wang");
  }
}
