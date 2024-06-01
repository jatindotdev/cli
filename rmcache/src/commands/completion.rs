use clap::CommandFactory;
use clap_complete::{generate, Shell};

use super::command::Command;
use crate::cli::CLI;
use std::io;

#[derive(clap::Parser, Debug)]
#[clap(about = "Generates shell completions")]
pub struct Completion {
    #[clap(long)]
    pub shell: Option<Shell>,
}

impl Command for Completion {
    type Error = std::io::Error;

    fn apply(self, _: &crate::flags::Flags) -> Result<(), Self::Error> {
        let mut cmd = CLI::command();
        cmd.build();

        let shell = self.shell.unwrap_or_else(|| {
            let shell = std::env::var("SHELL").unwrap_or_default();
            let shell = shell.split('/').last().unwrap_or_default();

            match shell {
                "bash" => Shell::Bash,
                "zsh" => Shell::Zsh,
                "fish" => Shell::Fish,
                "powershell" => Shell::PowerShell,
                "elvish" => Shell::Elvish,
                _ => {
                    eprintln!("Unsupported shell: {}", shell);
                    std::process::exit(1);
                }
            }
        });

        generate(shell, &mut cmd, env!("CARGO_PKG_NAME"), &mut io::stdout());

        Ok(())
    }
}
