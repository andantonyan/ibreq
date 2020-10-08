static BODY_SEPARATOR: &str = "\r\n\r\n";
static CONFIG_DECRYPT_CHAR_LEFT_SHIFT: u8 = 13;

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

fn decrypt(s: &str) -> String {
  return s
    .chars()
    .map(|c| (c as u8 - CONFIG_DECRYPT_CHAR_LEFT_SHIFT as u8) as char)
    .collect::<String>();
}
