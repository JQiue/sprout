use crate::{
  MASTER_URL,
  error::Error,
  helper::{draw_table, get_cli_config},
};

pub async fn list() -> Result<(), Error> {
  let token = get_cli_config()
    .token
    .ok_or(Error::AuthenticationRequired)?;
  let rpc = rpc::MasterRpc::new(MASTER_URL.to_string())?;
  let sites = rpc.get_sites(&token).await?.sites;
  let mut rows: Vec<Vec<String>> = sites
    .iter()
    .map(|site| {
      let site_id = site.site_id.clone();
      let name = site.name.clone();
      let bind_domain = site.domain.clone().unwrap_or("None".to_string());
      let status = site.status.to_string();
      vec![site_id, name, bind_domain, status]
    })
    .collect();
  rows.insert(
    0,
    vec![
      "Site ID".to_string(),
      "Name".to_string(),
      "Bind Domain".to_string(),
      "Status".to_string(),
    ],
  );
  draw_table(rows);
  Ok(())
}

#[cfg(test)]
mod test {
  #[test]
  fn test_console() {}
}
