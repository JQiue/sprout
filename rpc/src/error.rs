#[derive(Debug)]
pub enum AppError {
  Error,
  RpcCallError,
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<reqwest::Error> for AppError {
  fn from(err: reqwest::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::RpcCallError
  }
}
