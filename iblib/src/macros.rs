#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
  ($($arg:tt)+) => (println!($($arg)+))
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
  ($($arg:tt)+) => (std::convert::identity(format_args!($($arg)+)))
}

#[macro_export]
macro_rules! ternary {
  ($c:expr, $v:expr, $v1:expr) => {
      if $c {$v} else {$v1}
  };
}