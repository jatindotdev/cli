mod utils {
    pub mod log;
}

use clap::Parser;
use color_print::cprintln;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, process::exit};

#[derive(Parser, Debug)]
struct CLI {
    #[clap(short, long, help = "Prints verbose information")]
    verbose: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Options {
    disable: Option<Vec<String>>,
    only: Option<Vec<String>>,
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
            },
            paths: HashMap::new(),
            commands: HashMap::new(),
        }
    }
}

fn main() {
    let cli = CLI::parse();

    let config_path = {
        let mut path = dirs::home_dir().unwrap_or_else(|| {
            log_error!("Failed to get home directory, Make sure it exists.");
            exit(1);
        });
        path.push(".config");
        path.push("rmcache.toml");
        path
    };

    if !config_path.exists() {
        log_info!("Config file not found, creating one...");
        let config = Config::default();
        confy::store_path(&config_path, config).unwrap_or_else(|e| {
            log_error!("Failed to create config file: {}", e);
            exit(1);
        });
    }

    let config: Config = confy::load_path(config_path).unwrap_or_else(|e| match e {
        confy::ConfyError::BadTomlData(_) => {
            log_error!("Invalid Config file, Try deleting it and running the program again.");
            if cli.verbose {
                log_error!("{}", e.to_string());
            }
            exit(1);
        }
        _ => {
            log_error!("Failed to load config file: {}", e);
            exit(1);
        }
    });

    if cli.verbose {
        log_info!("Config file loaded successfully.");
    }

    if config.options.disable.is_some() && config.options.only.is_some() {
        log_error!("Both disable and only options are present, Please remove one.");
        exit(1);
    }

    let paths = get_paths(&config, &cli);

    dbg!(&config);
    dbg!(&cli);
    dbg!(paths);
}

fn get_paths<'a>(config: &'a Config, cli: &'a CLI) -> HashMap<&'a String, &'a Vec<String>> {
    if let Some(only) = &config.options.only {
        let mut paths: HashMap<&String, &Vec<String>> = HashMap::new();
        for (id, path) in &config.paths {
            if only.contains(&id) {
                paths.insert(id, path);
            }
        }

        if cli.verbose {
            log_info!("Enabled paths: {:?}", only);
        }
        return paths;
    }

    if let Some(disable) = &config.options.disable {
        let mut paths: HashMap<&String, &Vec<String>> = HashMap::new();
        for (id, path) in &config.paths {
            if !disable.contains(&id) {
                paths.insert(id, path);
            }
        }

        if cli.verbose {
            log_info!("Disabled paths: {:?}", disable);
        }
        return paths;
    }

    config.paths.iter().collect()
}
