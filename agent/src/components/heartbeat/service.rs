use actix_web::{get, post, web::Data, HttpResponse};

use crate::app::AppState;

#[post("/user")]
pub async fn register(state: Data<AppState>) -> HttpResponse {
  // let info = ServerInfo {
  //   agent_id: self.config.agent_id,
  //   hostname: self.get_hostname()?,
  //   ip: self.get_ip()?,
  //   cpu_info: self.get_cpu_info()?,
  //   memory_total: self.get_memory_total()?,
  //   disk_total: self.get_disk_total()?,
  //   os_info: self.get_os_info()?,
  // };
}

#[get("/heartbeat")]
pub async fn heartbeat(state: Data<AppState>) -> HttpResponse {}
