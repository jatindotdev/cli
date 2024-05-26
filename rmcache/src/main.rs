mod utils {
    pub mod log;
}

use clap::Parser;
use color_print::cprintln;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs, process::exit};

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(arg_required_else_help = true)]
struct CLI {
    #[clap(short, long, help = "Clears the cache")]
    clear: bool,

    #[clap(
        long,
        help = "Does not remove any files, only prints what would be removed"
    )]
    dry_run: bool,

    #[clap(
        short,
        long,
        help = "Clears the cache for the specified id",
        value_delimiter = ',',
        num_args = 1..
    )]
    only: Option<Vec<String>>,

    #[clap(
        short,
        long,
        help = "Clears the cache for all ids except the specified ones",
        value_delimiter = ',',
        num_args = 1..
    )]
    disable: Option<Vec<String>>,

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

    let only = match &cli.only {
        Some(only) => Some(only.clone()),
        None => config.options.only.clone(),
    };

    let disable = match &cli.disable {
        Some(disable) => Some(disable.clone()),
        None => config.options.disable.clone(),
    };

    if cli.verbose {
        if cli.only.is_some() && cli.disable.is_some() {
            log_warn!("Both only and disable flags are provided, only flag will be used.");
        }

        if cli.only.is_some() && config.options.only.is_some() {
            log_warn!(
                "Both only flag and config only option are provided, only flag will be used."
            );
        }

        if cli.disable.is_some() && config.options.disable.is_some() {
            log_warn!("Both disable flag and config disable option are provided, disable flag will be used.");
        }

        if let Some(only) = &only {
            log_info!("Enabled paths: {:?}", only);
        }

        if let Some(disable) = &disable {
            log_info!("Disabled paths: {:?}", disable);
        }
    }

    let options = Options { disable, only };
    let paths = config
        .paths
        .iter()
        .filter(|(id, _)| {
            if let Some(only) = &options.only {
                only.contains(id)
            } else if let Some(disable) = &options.disable {
                !disable.contains(id)
            } else {
                true
            }
        })
        .map(|(id, path)| {
            let mut new_path = Vec::new();
            for p in path.iter() {
                let p = fs::canonicalize(p);
                if let Ok(p) = p {
                    new_path.push(p);
                }
            }
            (id, new_path)
        })
        .collect::<HashMap<_, _>>();

    if paths.is_empty() {
        log_info!("No paths to clear.");
        exit(0);
    }

    if cli.verbose {
        log_info!("Paths to clear:");
        for (id, path) in &paths {
            log_info!("{}: {:?}", id, path);
        }
    }

    if cli.clear {
        for (id, path) in &paths {
            log_working!("Cleaning {}", id);

            if cli.dry_run {
                log_info!("Paths to remove:");
                for p in path {
                    log_pointer!("{}", p.display());
                }
                continue;
            }

            let mut failed = false;
            for p in path {
                match fs::read_dir(p) {
                    Ok(entries) => {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                if let Err(_) = remove_entry(entry, &cli) {
                                    failed = true;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        failed = true;
                        if cli.verbose {
                            log_error!("Failed to read directory: {:?}", p);
                        }
                    }
                }
            }
            if failed {
                log_info!("Some files were not removed.")
            }
        }
    }

    log_success!("Done.");
}

fn remove_entry(entry: fs::DirEntry, cli: &CLI) -> Result<(), ()> {
    let path = entry.path();
    let file_type = entry.file_type().map_err(|_| {
        if cli.verbose {
            log_error!("Failed to get file type: {:?}", path);
        }
        ()
    })?;

    if file_type.is_dir() {
        fs::remove_dir_all(&path).map_err(|_| {
            if cli.verbose {
                log_error!("Failed to remove directory: {:?}", path);
            }
            ()
        })?;
    } else if file_type.is_file() {
        fs::remove_file(&path).map_err(|_| {
            if cli.verbose {
                log_error!("Failed to remove file: {:?}", path);
            }
            ()
        })?;
    }

    Ok(())
}
