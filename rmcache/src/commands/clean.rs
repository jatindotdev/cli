use crate::cprintln;
use crate::flags;
use crate::os_utils::remove_entry;
use crate::Config;
use crate::{log_error, log_info, log_pointer, log_success, log_warn, log_working};
use std::process::exit;
use std::{collections::HashMap, fs};

#[derive(clap::Parser, Debug)]
pub struct Clean {
    #[clap(
        long,
        help = "Does not remove any files, only prints what would be removed"
    )]
    pub dry_run: bool,

    #[clap(short, long, help = "The shell to use for executing commands")]
    pub shell: Option<String>,

    #[clap(
      short,
      long,
      help = "Clears the cache for the specified id",
      value_delimiter = ',',
      num_args = 1..
    )]
    pub only: Option<Vec<String>>,

    #[clap(
      short,
      long,
      help = "Clears the cache for all ids except the specified ones",
      value_delimiter = ',',
      num_args = 1..
    )]
    pub disable: Option<Vec<String>>,
}

impl super::command::Command for Clean {
    type Error = std::io::Error;

    fn apply(self, flags: &flags::Flags) -> Result<(), Self::Error> {
        let config_path = {
            let mut path = match dirs::home_dir() {
                Some(path) => path,
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Failed to get config directory.",
                    ));
                }
            };
            path.push(".config");
            path.push("rmcache.toml");
            path
        };

        if !config_path.exists() {
            log_info!("Config file not found, creating one...");
            let config = Config::default();
            if let Err(e) = confy::store_path(&config_path, config) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create config file: {}", e),
                ));
            }
        }

        let config: Config = match confy::load_path(&config_path) {
            Ok(config) => config,
            Err(e) => match e {
                confy::ConfyError::BadTomlData(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid Config file, Try deleting it and running the program again.",
                    ));
                }
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to load config file: {}", e),
                    ));
                }
            },
        };

        if flags.verbose {
            log_info!("Config file loaded successfully.");
        }

        let only = match &self.only {
            Some(only) => Some(only.clone()),
            None => config.options.only.clone(),
        };

        let disable = match &self.disable {
            Some(disable) => Some(disable.clone()),
            None => config.options.disable.clone(),
        };

        let shell = match &self.shell {
            Some(shell) => shell.clone(),
            None => config.options.shell.unwrap_or("bash".to_string()),
        };

        if flags.verbose {
            if self.only.is_some() && self.disable.is_some() {
                log_warn!("Both only and disable self are provided, only flag will be used.");
            }

            if self.only.is_some() && config.options.only.is_some() {
                log_warn!(
                    "Both only flag and config only option are provided, only flag will be used."
                );
            }

            if self.disable.is_some() && config.options.disable.is_some() {
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

        if flags.verbose {
            log_info!("Paths to clear:");
            for (id, path) in &paths {
                log_info!("{}: {:?}", id, path);
            }
        }

        for (id, path) in &paths {
            if path.is_empty() {
                continue;
            }

            log_working!("Cleaning {}", id);

            if self.dry_run {
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
                                if let Err(_) = remove_entry(&entry) {
                                    if flags.verbose {
                                        log_error!("Failed to remove file: {:?}", entry.path());
                                    }
                                    failed = true;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        failed = true;
                        if flags.verbose {
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

            if self.dry_run {
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

                        if flags.verbose {
                            log_error!("Command: {}", cmd);
                            log_error!("Exit code: {}", status);
                        }
                    }
                }
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to execute command: {}", id),
                    ));
                }
            }
        }

        log_success!("Done.");

        Ok(())
    }
}
