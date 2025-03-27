use serde_json::{Value, json};
use sysinfo::{CpuRefreshKind, RefreshKind};

use crate::{app::AppState, error::AppError};

pub async fn heartbeat(_state: &AppState) -> Result<Value, AppError> {
  let mut s = sysinfo::System::new_with_specifics(
    RefreshKind::everything().with_cpu(CpuRefreshKind::everything()),
  );
  // Wait a bit because CPU usage is based on diff.
  std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
  // Refresh CPUs again to get actual value.
  s.refresh_memory();
  s.refresh_cpu_usage();
  Ok(json!({
    "total_memory": s.total_memory() / 1024 / 1024,
    "free_memory": s.free_memory() / 1024 / 1024,
    "memory_usage": ((s.used_memory() as f64 / s.total_memory() as f64) * 100.0).trunc(),
    "cpu_cores": s.cpus().len(),
    "cpu_usage": s.global_cpu_usage().trunc(),
  }))
}
