fn main() {
    // TODO: override from outside
  println!("cargo:rustc-env=CONF_HOST=localhost");
  println!("cargo:rustc-env=CONF_PORT=3000");
  println!("cargo:rustc-env=CONF_PATH=/");
  println!("cargo:rustc-env=CONF_METHOD=GET");
  println!("cargo:rustc-env=CONF_SSL=false");
  // TODO: use absolute path
  println!("cargo:rustc-env=IMAGE_PLACEHOLDER_PATH=placeholder.jpg");
}
