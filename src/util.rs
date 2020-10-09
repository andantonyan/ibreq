use crate::{config::ConfigMap, constant::*};
use rand::Rng;
use std::collections::HashMap;

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
