#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
      cprintln!("<blue>[INFO]</> {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
      cprintln!("<red>[ERROR]</> {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
      cprintln!("<yellow>[WARN]</> {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_success {
  ($($arg:tt)*) => {
    cprintln!("<green>[SUCCESS]</> {}", format!($($arg)*))
  };
}

#[macro_export]
macro_rules! log_working {
  ($($arg:tt)*) => {
    cprintln!("<magenta>[WORKING]</> {}", format!($($arg)*))
  };
}

#[macro_export]
macro_rules! log_pointer {
  ($($arg:tt)*) => {
    cprintln!("<blue,bold>\t•</> {}", format!($($arg)*))
  };
}
