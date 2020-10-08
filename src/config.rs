use core::str::FromStr;
use std::collections::HashMap;

static CONFIG_SEPARATOR: &str = ";";
static CONFIG_PAIR_SEPARATOR: &str = "=";

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

#[derive(Debug, Clone)]
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
      config_map.safe_get("headers", "GET / HTTP/1.1\nAccept: */*".into()),
      config_map.safe_get("content_length", 1024),
      config_map.safe_get("thread_count", 10),
      config_map.safe_get("call_interval_in_ms", 100),
      config_map.safe_get("config_fetch_interval_in_ms", 3600000),
      config_map.safe_get("host", "localhost".into()),
      config_map.safe_get("port", 80),
      config_map.safe_get("enabled", false),
      config_map.safe_get("ssl", false),
    )
  }
}

#[derive(Debug, Clone, Default)]
pub struct AppConfig {
  pub original_path: String,
  pub token: String,
}

impl AppConfig {
  pub fn new(token: String, original_path: String) -> AppConfig {
    AppConfig {
      token,
      original_path,
    }
  }
}

impl From<ConfigMap> for AppConfig {
  fn from(config_map: ConfigMap) -> AppConfig {
    AppConfig::new(
      config_map.safe_get("token", "".into()),
      config_map.safe_get("original_path", "".into()),
    )
  }
}

pub fn parse_config(s: &str) -> ConfigMap {
  let parsed: ConfigMap = s
    .split(CONFIG_SEPARATOR)
    .map(|line: &str| line.split(CONFIG_PAIR_SEPARATOR).collect())
    .collect::<Vec<Vec<&str>>>()
    .iter()
    .fold(HashMap::<String, String>::new(), |mut conf, pair| {
      if pair.len() == 2 {
        conf.insert(pair[0].trim().to_string(), pair[1].trim().to_string());
      };
      return conf;
    });

  return parsed;
}
