use iblib::util::*;

#[derive(Debug, Clone, Default)]
pub struct ControllerConfig {
  pub host: String,
  pub port: u16,
  pub path: String,
  pub method: String,
  pub user_agents: Vec<String>,
  pub referers: Vec<String>,
  pub content_length: u32,
  pub raw_headers: Vec<String>,
  pub raw_bodies: Vec<String>,
  pub enabled: bool,
  pub thread_count: u16,
  pub call_interval_in_ms: u64,
  pub config_fetch_interval_in_ms: u64,
  pub ssl: bool,
}

impl ControllerConfig {
  pub fn new(
    host: String,
    port: u16,
    path: String,
    method: String,
    user_agents: Vec<String>,
    referers: Vec<String>,
    content_length: u32,
    raw_headers: Vec<String>,
    raw_bodies: Vec<String>,
    enabled: bool,
    thread_count: u16,
    call_interval_in_ms: u64,
    config_fetch_interval_in_ms: u64,
    ssl: bool,
  ) -> ControllerConfig {
    ControllerConfig {
      host,
      port,
      path,
      method,
      user_agents,
      referers,
      content_length,
      raw_headers,
      raw_bodies,
      enabled,
      thread_count,
      call_interval_in_ms,
      config_fetch_interval_in_ms,
      ssl,
    }
  }

  pub fn get_addr(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }
}

impl From<ConfigMap> for ControllerConfig {
  fn from(config_map: ConfigMap) -> ControllerConfig {
    ControllerConfig::new(
      config_map.parse("host"),
      config_map.parse("port"),
      config_map.parse("path"),
      config_map.parse("method"),
      config_map.parse_vec("user_agents"),
      config_map.parse_vec("referers"),
      config_map.parse("content_length"),
      config_map.parse_vec("raw_headers"),
      config_map.parse_vec("raw_bodies"),
      config_map.parse("enabled"),
      config_map.parse("thread_count"),
      config_map.parse("call_interval_in_ms"),
      config_map.parse("config_fetch_interval_in_ms"),
      config_map.parse("ssl"),
    )
  }
}

#[derive(Debug, Clone, Default)]
pub struct AppConfig {}

impl AppConfig {
  pub fn new() -> AppConfig {
    AppConfig {}
  }
}

impl From<ConfigMap> for AppConfig {
  fn from(_: ConfigMap) -> AppConfig {
    AppConfig::new()
  }
}
