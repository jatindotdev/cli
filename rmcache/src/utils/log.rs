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
