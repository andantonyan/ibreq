pub mod config;

use config::ControllerConfig;
use iblib::{connection::create_stream, constant::*, debug, response::Response, util::*};

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
  let conf = parse_config(&res.decrypted_body());
  let conf = ControllerConfig::from(conf);

  debug!("Done fetching config {:?}.", conf);

  Ok(conf)
}

pub fn call(conf: &ControllerConfig) -> Result<Response> {
  let mut stream = create_stream(conf.ssl, &conf.host, conf.port)?;
  let body = generate_body(conf);

  stream.write_buf(conf.headers.as_bytes())?;
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

pub fn generate_body(conf: &ControllerConfig) -> Vec<u8> {
  let body: Vec<u8>;

  if !conf.body.is_empty() {
    body = conf.body.as_bytes().to_vec();
  } else {
    body = vec![0; conf.content_length as usize]
      .iter()
      .map(|_| gen_random_byte())
      .collect();
  }

  body
}
