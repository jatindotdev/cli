use crate::{flags::Flags, log_error};
use color_print::cprintln;

pub trait Command: Sized {
    type Error: std::error::Error;
    fn apply(self, config: &Flags) -> Result<(), Self::Error>;

    fn handle_error(err: Self::Error, _: &Flags) {
        log_error!("{}", err);
        std::process::exit(1);
    }

    fn call(self, config: Flags) {
        match self.apply(&config) {
            Ok(()) => (),
            Err(err) => Self::handle_error(err, &config),
        }
    }
}
