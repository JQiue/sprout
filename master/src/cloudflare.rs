use std::net::Ipv4Addr;

use cloudflare::endpoints::dns::dns::{self, CreateDnsRecordParams};
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::client::ClientConfig;
use cloudflare::framework::client::async_api::Client;
use cloudflare::framework::{Environment, OrderDirection};

struct Cloudflare {
  api_client: Client,
  zone_identifier: String,
}

impl Cloudflare {
  async fn new(zone_identifier: String, email: String, key: String) -> Self {
    let credentials = Credentials::UserAuthKey { email, key };
    let api_client = Client::new(
      credentials,
      ClientConfig::default(),
      Environment::Production,
    )
    .unwrap();
    Self {
      api_client,
      zone_identifier,
    }
  }

  pub async fn dns(&self) {
    let endpoint = dns::ListDnsRecords {
      zone_identifier: &self.zone_identifier,
      params: dns::ListDnsRecordsParams {
        direction: Some(OrderDirection::Ascending),
        ..Default::default()
      },
    };
    let response = self.api_client.request(&endpoint).await.unwrap();
    println!("{:#?}", response);
  }

  pub async fn create_a_record(&self, name: &str, ip: Ipv4Addr) {
    let endpoint = dns::CreateDnsRecord {
      zone_identifier: &self.zone_identifier,
      params: CreateDnsRecordParams {
        ttl: None,
        priority: None,
        proxied: None,
        name,
        content: dns::DnsContent::A { content: ip },
      },
    };
    let response = self.api_client.request(&endpoint).await.unwrap();
    println!("{:#?}", response);
  }

  pub async fn create_cname_record(&self, name: &str, content: String) {
    let endpoint = dns::CreateDnsRecord {
      zone_identifier: &self.zone_identifier,
      params: CreateDnsRecordParams {
        ttl: None,
        priority: None,
        proxied: None,
        name,
        content: dns::DnsContent::CNAME { content },
      },
    };
    let response = self.api_client.request(&endpoint).await.unwrap();
    println!("{:#?}", response);
  }
}

#[cfg(test)]
mod test {

  use crate::config::Config;

  use super::*;

  #[tokio::test]
  pub async fn test_dns() {
    let Config {
      cloudflare_api_key,
      cloudflare_email,
      cloudflare_zone_id,
      ..
    } = Config::from_env().unwrap();
    let cf = Cloudflare::new(cloudflare_zone_id, cloudflare_email, cloudflare_api_key).await;
    cf.dns().await;
  }

  #[tokio::test]
  pub async fn test_create_cname_record() {
    let Config {
      cloudflare_api_key,
      cloudflare_email,
      cloudflare_zone_id,
      ..
    } = Config::from_env().unwrap();
    let cf = Cloudflare::new(cloudflare_zone_id, cloudflare_email, cloudflare_api_key).await;
    cf.create_cname_record("example", "root.is.me".to_string())
      .await;
  }
}
