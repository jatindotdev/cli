use crate::commands;
use crate::commands::command::Command;
use crate::flags::Flags;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Completions(commands::completions::Completions),
    Clean(commands::clean::Clean),
}

impl SubCommand {
    pub fn call(self, flags: Flags) {
        match self {
            SubCommand::Completions(cmd) => cmd.call(flags),
            SubCommand::Clean(cmd) => cmd.call(flags),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(arg_required_else_help = true)]
pub struct CLI {
    #[clap(flatten)]
    pub flags: Flags,

    #[clap(subcommand)]
    pub commands: SubCommand,
}

pub fn parse() -> CLI {
    CLI::parse()
}
