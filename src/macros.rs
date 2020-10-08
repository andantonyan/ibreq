#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
  ($($arg:tt)+) => (dbg!(format_args!($($arg)+)))
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
  ($($arg:tt)+) => (std::convert::identity(format_args!($($arg)+)))
}
