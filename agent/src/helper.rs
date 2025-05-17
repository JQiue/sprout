use serde::{Deserialize, Serialize};
use std::{
  fs,
  io::{BufRead, BufReader},
  net::IpAddr,
  path::PathBuf,
  process::{Command, Stdio},
};
use tracing::{debug, error, info, trace};

use crate::error::AppError;

pub fn extract_tar(filename: String, output: String) -> bool {
  let mut child = Command::new("tar")
    .arg("-xf")
    .arg(&filename)
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
    info!("{}: decompressed", filename);
    true
  } else {
    error!("Build failed with status: {}", status);
    false
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

  pub fn generate_config(&self, server_name: &str, root_path: &str, bandwidth: &str) -> String {
    let mut config = String::new();
    config.push_str("server {\n");
    config.push_str("    listen 80;\n");
    config.push_str(&format!("    server_name {};\n", server_name));
    config.push_str("    location / {\n");
    config.push_str("        try_files $uri $uri/ /index.html;\n");
    config.push_str(&format!("        root {};\n", root_path));
    config.push_str("        index index.html;\n");
    config.push_str(&format!("        limit_rate {};\n", bandwidth));
    config.push_str("    }\n");
    config.push_str("}\n");
    debug!("Nginx config: {}", config);
    config
  }

  pub fn remove_config(&self, site_id: &str) -> Result<bool, std::io::Error> {
    let domian_config = self.config_path.join(site_id.to_string() + ".conf");
    if domian_config.exists() {
      fs::remove_file(domian_config)?;
      Ok(true)
    } else {
      error!("Config file does not exist: {:?}", domian_config);
      Ok(false)
    }
  }

  pub fn deploy(&self, server_name: &str, root_path: &str, bandwidth: &str, site_id: &str) -> bool {
    // 生成配置文件内容
    let config_content = self.generate_config(server_name, root_path, bandwidth);

    // 暂时简单写入，以后替换为原子写入
    if let Err(e) = fs::create_dir_all(self.config_path.as_path()) {
      tracing::error!("Failed to create config directory: {}", e);
      return false;
    }

    if fs::write(
      self.config_path.join(format!("{}.conf", site_id)),
      config_content,
    )
    .map_err(|_| {
      tracing::error!("{}", "write Nginx configuration failed");
    })
    .is_err()
    {
      return false;
    }

    if self.ssl_enabled {
      self.apply_ssl();
    }
    // 测试配置是否正确
    if Command::new("nginx").arg("-t").status().is_err() {
      tracing::error!("{}", "Nginx configuration test failed");
      return false;
    }
    // 重新加载 Nginx
    if Command::new("nginx")
      .arg("-s")
      .arg("reload")
      .status()
      .is_err()
    {
      tracing::error!("{}", "reload Nginx failed");
      return false;
    }
    true
  }

  pub fn apply_ssl(&self) {}
}

pub fn check_dns_record(domian: &str, ip: IpAddr) -> Result<bool, AppError> {
  let ips = dns_lookup::lookup_host(domian)?;
  println!("{:?}", ips);
  if ips.is_empty() {
    Ok(false)
  } else {
    if ips.contains(&ip) {
      Ok(true)
    } else {
      Ok(false)
    }
  }
}

#[cfg(test)]
mod test {
  use super::{NginxConfig, check_dns_record};

  #[test]
  fn test_deploy() {
    let nc = NginxConfig::new("/etc/nginx/sprout", false);
    println!(
      "{}",
      nc.generate_config("jinqiu.wang", "/var/www/html", "100")
    );
    nc.deploy("jinqiu.wang", "/var/www/html", "100", "abcdefghijklmn");
  }

  #[test]
  fn test_revoke() {
    let nc = NginxConfig::new("/etc/nginx/sprout", false);
    nc.remove_config("abcdefghijklmn").unwrap();
  }

  #[test]
  fn test_check_dns_record() {
    let res = check_dns_record("localhost", "127.0.0.1".parse().unwrap()).unwrap();
    assert_eq!(res, true);
  }
}
