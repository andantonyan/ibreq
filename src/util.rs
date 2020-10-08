use rand::Rng;

static CONFIG_DECRYPT_CHAR_LEFT_SHIFT: u8 = 13;

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
