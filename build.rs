fn main() {
  println!(
    "cargo:rustc-env=CONF_HOST={}",
    option_env!("CONF_HOST").unwrap_or("localhost")
  );
  println!(
    "cargo:rustc-env=CONF_PORT={}",
    option_env!("CONF_PORT").unwrap_or("3000")
  );
  println!(
    "cargo:rustc-env=CONF_PATH={}",
    option_env!("CONF_PATH").unwrap_or("/")
  );
  println!(
    "cargo:rustc-env=CONF_METHOD={}",
    option_env!("CONF_METHOD").unwrap_or("GET")
  );
  println!(
    "cargo:rustc-env=CONF_SSL={}",
    option_env!("CONF_SSL").unwrap_or("false")
  );
  println!(
    "cargo:rustc-env=IMAGE_PLACEHOLDER_PATH={}",
    option_env!("IMAGE_PLACEHOLDER_PATH")
      .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/placeholder.jpg"))
  );
}
