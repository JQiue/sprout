use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("Internal error")]
  Internal {
    #[from]
    source: std::io::Error,
  },
  #[error("Api error")]
  Api(u16, i32, String),
  #[error("Cloudflare Framework error")]
  CloudflareFramework {
    #[from]
    source: cloudflare::framework::Error,
  },
  #[error("Cloudflare Response error")]
  CloudflareResponse {
    #[from]
    source: cloudflare::framework::response::ApiFailure,
  },
  #[error("Connect agent error: {0}")]
  ConnectAgent(String),
  #[error("Connect master error")]
  ConnectMaster,
  #[error("Build request error")]
  BuildRequest,
  #[error("Rpc call error: {source}")]
  RpcCall {
    #[source]
    source: reqwest::Error,
  },
  #[error("Invalid content type")]
  InvalidContentType,
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    if err.is_builder() {
      Error::BuildRequest
    } else if err.is_connect() {
      Error::ConnectAgent(err.url().unwrap().to_string())
    } else if err.is_decode() {
      Error::RpcCall { source: err }
    } else {
      Error::RpcCall { source: err }
    }
  }
}
