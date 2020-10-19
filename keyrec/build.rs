fn main() {
  println!(
    "cargo:rustc-env=KEYS_SAVE_INTERVAL_IN_MS={}",
    option_env!("KEYS_SAVE_INTERVAL_IN_MS").unwrap_or("5000")
  );
}
