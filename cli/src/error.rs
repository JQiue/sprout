#[derive(Debug)]
pub enum Error {
  RpcCall,
  AuthenticationRequired,
  CannotConnect,
}

impl From<rpc::error::Error> for Error {
  fn from(err: rpc::error::Error) -> Self {
    match err {
      rpc::error::Error::Internal { .. } => Error::RpcCall,
      rpc::error::Error::RpcCall { .. } => Error::RpcCall,
      rpc::error::Error::CloudflareFramework { .. } => Error::RpcCall,
      rpc::error::Error::CloudflareResponse { .. } => Error::RpcCall,
      rpc::error::Error::ConnectAgent { .. } => Error::RpcCall,
      rpc::error::Error::Api(status_code, code, msg) => {
        if status_code == 401 {
          return Error::AuthenticationRequired;
        }
        if status_code == 403 {
          return Error::AuthenticationRequired;
        }
        if status_code == 500 {
          return Error::CannotConnect;
        }
        return Error::RpcCall;
      }
      _ => Error::RpcCall,
    }
  }
}
