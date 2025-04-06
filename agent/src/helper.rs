use serde::{Deserialize, Serialize};
use std::{
  fs,
  io::{BufRead, BufReader},
  path::Path,
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
  domain: String,
  root_path: String,
  ssl_enabled: bool,
  ssl_config: Option<SslConfig>,
  custom_locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SslConfig {
  certificate_path: String,
  certificate_key_path: String,
  protocols: Vec<String>,
  ciphers: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
  path: String,
  try_files: Option<String>,
  root: Option<String>,
  proxy_pass: Option<String>,
}

impl NginxConfig {
  pub fn new(
    domain: String,
    root_path: String,
    ssl_enabled: bool,
    ssl_config: Option<SslConfig>,
  ) -> Self {
    Self {
      domain,
      root_path,
      ssl_enabled,
      ssl_config,
      custom_locations: vec![],
    }
  }
  pub fn with_ssl(mut self, ssl_config: SslConfig) -> Self {
    self.ssl_enabled = true;
    self.ssl_config = Some(ssl_config);
    self
  }
  pub fn add_location(mut self, location: Location) -> Self {
    self.custom_locations.push(location);
    self
  }

  pub fn generate_config(&self) -> String {
    let mut config = String::new();
    // HTTP server block
    config.push_str("server {\n");
    config.push_str("    listen 80;\n");
    if self.ssl_enabled {
      config.push_str("    listen 443 ssl;\n");
      // SSL redirect
      config.push_str(&format!(
        "    if ($server_port !~ 443) {{\n        rewrite ^(.*)$ https://{} permanent;\n    }}\n",
        self.domain
      ));
      // SSL configuration
      if let Some(ssl) = &self.ssl_config {
        config.push_str(&format!("    ssl_certificate {};\n", ssl.certificate_path));
        config.push_str(&format!(
          "    ssl_certificate_key {};\n",
          ssl.certificate_key_path
        ));
        config.push_str("    ssl_session_timeout 5m;\n");
        config.push_str(&format!("    ssl_protocols {};\n", ssl.protocols.join(" ")));
        config.push_str(&format!("    ssl_ciphers {};\n", ssl.ciphers));
        config.push_str("    ssl_prefer_server_ciphers on;\n");
      }
    }
    config.push_str(&format!("    server_name {};\n", self.domain));
    // Default location
    config.push_str("    location / {\n");
    config.push_str("        try_files $uri $uri/ /index.html;\n");
    config.push_str(&format!("        root {};\n", self.root_path));
    config.push_str("        index index.html;\n");
    config.push_str("    }\n");
    // Custom locations
    for location in &self.custom_locations {
      config.push_str(&format!("    location {} {{\n", location.path));
      if let Some(try_files) = &location.try_files {
        config.push_str(&format!("        try_files {};\n", try_files));
      }
      if let Some(root) = &location.root {
        config.push_str(&format!("        root {};\n", root));
      }
      if let Some(proxy_pass) = &location.proxy_pass {
        config.push_str(&format!("        proxy_pass {};\n", proxy_pass));
      }
      config.push_str("    }\n");
    }
    config.push_str("}\n");
    config
  }
  pub fn deploy(&self, config_path: &Path) -> bool {
    // 创建配置文件
    let config_content = self.generate_config();

    // 暂时简单写入，以后替换为原子接入
    if let Err(e) = fs::create_dir_all(config_path) {
      tracing::error!("Failed to create config directory: {}", e);
      return false;
    }

    if !fs::write(
      config_path.join(format!("{}.conf", self.domain)),
      config_content,
    )
    .map_err(|_| {
      tracing::error!("{}", "write Nginx configuration failed");
    })
    .is_ok()
    {
      return false;
    }

    // 测试配置是否有效
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
}

#[cfg(test)]
mod test {

  #[test]
  pub fn test() {}
}
