mod cli;
mod commands;
mod flags;
mod log;
mod os_utils;

use color_print::cprintln;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Options {
    disable: Option<Vec<String>>,
    only: Option<Vec<String>>,
    shell: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    options: Options,
    paths: HashMap<String, Vec<String>>,
    commands: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            options: Options {
                disable: Option::None,
                only: Option::None,
                shell: Option::Some("bash".to_string()),
            },
            paths: HashMap::new(),
            commands: HashMap::new(),
        }
    }
}

fn main() {
    let cli = crate::cli::parse();
    cli.commands.call(cli.flags);
}
