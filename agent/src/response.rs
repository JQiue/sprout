use std::fmt::{Debug, Display};

use actix_web::{
  error::{self},
  http::{self},
  HttpResponse,
};
use serde::Serialize;

/// 响应状态码枚举
#[derive(Debug, Clone, Copy, Serialize)]
pub enum StatusCode {
  /// 成功
  Success = 0,
  // 客户端错误 (1000-1999)
  /// 参数错误
  ParamError = 1000,
  /// 认证失败
  UploadTokenError = 1001,

  /// 服务器内部错误
  ServerError = 2000,
  /// 文件系统错误
  FileSystemError = 2001,
  /// 关键字加载失败
  LoadKeywordsError = 2002,
  /// 发送 Master 请求失败
  SendMasterRequestError = 2003,

  /// 部署错误 (3000-3999)
  /// 部署失败
  DeployError = 3000,
  /// 文件上传失败
  UploadError = 3001,
  /// Git 克隆失败
  CloneError = 3002,
}

impl StatusCode {
  pub fn message(&self) -> &str {
    match self {
      StatusCode::Success => "操作成功",
      StatusCode::ParamError => "参数错误",
      StatusCode::FileSystemError => "文件系统错误",
      StatusCode::DeployError => "部署失败",
      StatusCode::CloneError => "Git 克隆失败",
      StatusCode::UploadError => "文件上传失败",
      StatusCode::ServerError => "服务器内部错误",
      StatusCode::UploadTokenError => "认证失败",
      StatusCode::LoadKeywordsError => "关键字加载失败",
      StatusCode::SendMasterRequestError => "发送 Master 请求失败",
    }
  }
}

#[derive(Debug, Serialize)]
pub struct Response<T>
where
  T: Debug,
{
  pub code: i32,
  pub msg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

impl<T> Response<T>
where
  T: Debug,
{
  pub fn success(data: T) -> Self {
    Response {
      code: StatusCode::Success as i32,
      msg: StatusCode::Success.message().to_string(),
      data: Some(data),
    }
  }

  pub fn error(code: StatusCode) -> Self {
    Response {
      code: code as i32,
      msg: code.message().to_string(),
      data: None,
    }
  }

  pub fn error_with_msg(code: StatusCode, msg: String) -> Self {
    Response {
      code: code as i32,
      msg,
      data: None,
    }
  }
}

impl<T> Display for Response<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, r#"{{ "code": {}, "msg": "{}" }}"#, self.code, self.msg)
  }
}

impl<T> error::ResponseError for Response<T>
where
  T: Debug,
{
  fn error_response(&self) -> HttpResponse {
    HttpResponse::Ok().json(Response::<()>::error_with_msg(
      StatusCode::ServerError,
      "测试错误".to_string(),
    ))
  }

  fn status_code(&self) -> http::StatusCode {
    http::StatusCode::INTERNAL_SERVER_ERROR
  }
}
