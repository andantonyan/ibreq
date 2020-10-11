use core::str::FromStr;
use std::collections::HashMap;

pub type ConfigMap = HashMap<String, String>;

pub trait ConfigManager<'a> {
  fn safe_get<T>(&self, key: &'a str, default_value: T) -> T
  where
    T: ToString + FromStr;
}

impl<'a> ConfigManager<'a> for ConfigMap {
  fn safe_get<T>(&self, key: &'a str, default_value: T) -> T
  where
    T: ToString + FromStr,
  {
    return self
      .get(key)
      .unwrap_or(&default_value.to_string())
      .parse::<T>()
      .unwrap_or(default_value);
  }
}

#[derive(Debug, Clone, Default)]
pub struct ControllerConfig {
  pub headers: String,
  pub host: String,
  pub port: u16,
  pub content_length: u32,
  pub thread_count: u16,
  pub call_interval_in_ms: u64,
  pub config_fetch_interval_in_ms: u64,
  pub enabled: bool,
  pub ssl: bool,
}

impl ControllerConfig {
  pub fn new(
    headers: String,
    content_length: u32,
    thread_count: u16,
    call_interval_in_ms: u64,
    config_fetch_interval_in_ms: u64,
    host: String,
    port: u16,
    enabled: bool,
    ssl: bool,
  ) -> ControllerConfig {
    ControllerConfig {
      headers,
      content_length,
      thread_count,
      call_interval_in_ms,
      config_fetch_interval_in_ms,
      host,
      port,
      enabled,
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
      config_map.safe_get("headers", "".into()),
      config_map.safe_get("content_length", 1024),
      config_map.safe_get("thread_count", 10),
      config_map.safe_get("call_interval_in_ms", 100),
      config_map.safe_get("config_fetch_interval_in_ms", 3600000),
      config_map.safe_get("host", "".into()),
      config_map.safe_get("port", 80),
      config_map.safe_get("enabled", false),
      config_map.safe_get("ssl", false),
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
