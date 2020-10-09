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

  let image_placeholder_path = option_env!("IMAGE_PLACEHOLDER_PATH")
    .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/placeholder.jpg"));
  println!(
    "cargo:rustc-env=IMAGE_PLACEHOLDER_PATH={}",
    image_placeholder_path
  );

  pack();
}

#[cfg(windows)]
fn pack() {
  let mut res = winres::WindowsResource::new();
  res.set_icon("test.ico");
  res.compile().unwrap();
}

#[cfg(not(windows))]
fn pack() {}
