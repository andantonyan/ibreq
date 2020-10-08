use crate::util::decrypt;

static BODY_SEPARATOR: &str = "\r\n\r\n";

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

    if res_chunks.len() >= 2 {
      Response::new(res_chunks[0].into(), res_chunks[1].into())
    } else {
      Response::default()
    }
  }
}
