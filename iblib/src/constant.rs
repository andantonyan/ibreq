use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub const CONF_HOST: &str = env!("CONF_HOST");
pub const CONF_PORT: &str = env!("CONF_PORT");
pub const CONF_PATH: &str = env!("CONF_PATH");
pub const CONF_METHOD: &str = env!("CONF_METHOD");
pub const CONF_SSL: &str = env!("CONF_SSL");
pub const PLACEHOLDER_BUF: &'static [u8] = include_bytes!(env!("IMAGE_PLACEHOLDER_PATH"));

pub const CR: &str = "\r";
pub const LF: &str = "\n";
pub const CRLF: &str = "\r\n";
pub const BODY_SEPARATOR: &str = "\r\n\r\n";
pub const MAX_BUFFER_CHUNK_SIZE: u32 = 1024;
pub const ENCRYPTION_CHAR_SHIFT: u8 = 13;
pub const CONFIG_SEPARATOR: &str = ";;;";
pub const CONFIG_PAIR_SEPARATOR: &str = "===";
pub static TOKEN: Lazy<String> =
  Lazy::new(|| thread_rng().sample_iter(&Alphanumeric).take(32).collect());
