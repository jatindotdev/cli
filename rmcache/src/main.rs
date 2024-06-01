mod cli;
mod log;
mod os_utils;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use cli::{Commands, CLI};
use color_print::cprintln;
use os_utils::remove_entry;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, process::exit};

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
    let cli = CLI::parse();

    match cli.commands {
        Some(Commands::Completion { shell }) => {
            let mut cmd = CLI::command();
            cmd.build();

            let shell = shell.unwrap_or_else(|| {
                if let Some(shell) = std::env::var("SHELL").ok() {
                    if shell.contains("bash") {
                        Shell::Bash
                    } else if shell.contains("zsh") {
                        Shell::Zsh
                    } else if shell.contains("fish") {
                        Shell::Fish
                    } else if shell.contains("elvish") {
                        Shell::Elvish
                    } else if shell.contains("powershell") {
                        Shell::PowerShell
                    } else {
                        log_error!("Unsupported shell: {}", shell);
                        exit(1);
                    }
                } else {
                    log_error!("Failed to get shell from environment, Make sure SHELL is set.");
                    exit(1);
                }
            });
            generate(shell, &mut cmd, env!("CARGO_PKG_NAME"), &mut io::stdout());
        }
        _ => {
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
                    log_error!(
                        "Invalid Config file, Try deleting it and running the program again."
                    );
                    if cli.flags.verbose {
                        log_error!("{}", e.to_string());
                    }
                    exit(1);
                }
                _ => {
                    log_error!("Failed to load config file: {}", e);
                    exit(1);
                }
            });

            if cli.flags.verbose {
                log_info!("Config file loaded successfully.");
            }

            let only = match &cli.flags.only {
                Some(only) => Some(only.clone()),
                None => config.options.only.clone(),
            };

            let disable = match &cli.flags.disable {
                Some(disable) => Some(disable.clone()),
                None => config.options.disable.clone(),
            };

            let shell = match &cli.flags.shell {
                Some(shell) => shell.clone(),
                None => config.options.shell.unwrap_or("bash".to_string()),
            };

            if cli.flags.verbose {
                if cli.flags.only.is_some() && cli.flags.disable.is_some() {
                    log_warn!("Both only and disable flags are provided, only flag will be used.");
                }

                if cli.flags.only.is_some() && config.options.only.is_some() {
                    log_warn!(
                        "Both only flag and config only option are provided, only flag will be used."
                    );
                }

                if cli.flags.disable.is_some() && config.options.disable.is_some() {
                    log_warn!("Both disable flag and config disable option are provided, disable flag will be used.");
                }

                if let Some(only) = &only {
                    log_info!("Enabled paths: {:?}", only);
                }

                if let Some(disable) = &disable {
                    log_info!("Disabled paths: {:?}", disable);
                }
            }

            let paths = config
                .paths
                .iter()
                .filter(|(id, _)| {
                    if let Some(only) = &only {
                        only.contains(id)
                    } else if let Some(disable) = &disable {
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

            let commands = config.commands.iter().filter(|(id, _)| {
                if let Some(only) = &only {
                    only.contains(id)
                } else if let Some(disable) = &disable {
                    !disable.contains(id)
                } else {
                    true
                }
            });

            if paths.is_empty() && commands.clone().count() == 0 {
                log_info!("No paths or commands to clear.");
                exit(0);
            }

            if cli.flags.verbose {
                log_info!("Paths to clear:");
                for (id, path) in &paths {
                    log_info!("{}: {:?}", id, path);
                }
            }

            if cli.flags.clear {
                for (id, path) in &paths {
                    if path.is_empty() {
                        continue;
                    }

                    log_working!("Cleaning {}", id);

                    if cli.flags.dry_run {
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
                                if cli.flags.verbose {
                                    log_error!("Failed to read directory: {:?}", p);
                                }
                            }
                        }
                    }
                    if failed {
                        log_info!("Some files were not removed.")
                    }
                }

                for (id, cmd) in commands {
                    log_working!("Executing {}", id);

                    if cli.flags.dry_run {
                        log_pointer!("{}", cmd);
                        continue;
                    }

                    let status = std::process::Command::new(&shell)
                        .arg("-c")
                        .arg(cmd)
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .stdin(std::process::Stdio::null())
                        .status();

                    match status {
                        Ok(status) => {
                            if !status.success() {
                                log_error!("Failed to execute command {}", id);

                                if cli.flags.verbose {
                                    log_error!("Command: {}", cmd);
                                    log_error!("Exit code: {}", status);
                                }
                            }
                        }
                        Err(e) => {
                            log_error!("Failed to execute command: {}", cmd);
                            if cli.flags.verbose {
                                log_error!("{}", e);
                            }
                        }
                    }
                }
            }

            log_success!("Done.");
        }
    }
}
