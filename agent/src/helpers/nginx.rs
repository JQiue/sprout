use std::{fs, io, path::Path};

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

// 使用示例：统计文件大小
pub fn calculate_total_size(path: &Path) -> io::Result<u64> {
  let mut total_size = 0;
  let mut size_callback = |path: &Path| -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    total_size += metadata.len();
    Ok(())
  };

  visit_dirs(path, &mut size_callback)?;
  Ok(total_size)
}

pub fn create_nginx_server_80(domain: &str) -> Result<(), std::io::Error> {
  fs::write(
    format!("./{}.conf", domain),
    format!(
      "server {{
  listen 80;
  server_name {domain};
  location / {{
    try_files $uri $uri/ /404.html;
    root /root/jqiue-notes/dist;
    index index.html;
  }}
}}"
    ),
  )
}

pub fn create_nginx_server_443(domain: &str) -> Result<(), std::io::Error> {
  fs::write(
    format!("./{}.conf", domain),
    format!(
      "server {{
  listen 80;
  listen 443 ssl;
  server_name {domain};
  if ($server_port !~ 443) {{
    rewrite ^(.*)$ https://{domain} permanent;
  }}
  ssl_certificate crts/jinqiu.wang_bundle.pem;
  ssl_certificate_key crts/jinqiu.wang.key;
  ssl_session_timeout 5m;
  ssl_protocols TLSv1 TLSv1.1 TLSv1.2;
  ssl_ciphers ALL:!ADH:!EXPORT56:RC4+RSA:+HIGH:+MEDIUM:+LOW:+SSLv2:+EXP;
  ssl_prefer_server_ciphers on;
  location / {{
    try_files $uri $uri/ /404.html;
    root /root/jqiue-notes/dist;
    index index.html;
  }}
}}"
    ),
  )
}

pub fn test() {
  let path = Path::new("./sprout/dist");
  println!("{:?}", path);
  let total_size = calculate_total_size(path);
  println!("{:?}", total_size);
  create_nginx_server_80("jinqiu.wang");
  create_nginx_server_443("jinqiu.wang");
}
