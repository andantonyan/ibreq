use crate::{
  config::ConfigMap, CONFIG_DECRYPT_CHAR_LEFT_SHIFT, CONFIG_PAIR_SEPARATOR, CONFIG_SEPARATOR,
  DEFAULT_PLACEHOLDER_PATH,
};
use rand::Rng;
use std::{collections::HashMap, fs};

pub fn gen_random_byte() -> u8 {
  let mut rng = rand::thread_rng();
  rng.gen::<u8>()
}

pub fn decrypt(s: &str) -> String {
  return s
    .chars()
    .map(|c| (c as u8 - CONFIG_DECRYPT_CHAR_LEFT_SHIFT as u8) as char)
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

pub fn get_placeholder_buf() -> Vec<u8> {
  let path: &'static str =
    option_env!("IMAGE_PLACEHOLDER_PATH").unwrap_or(DEFAULT_PLACEHOLDER_PATH);

  return fs::read(path).unwrap();
}
