use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(arg_required_else_help = true)]
pub struct CLI {
    #[clap(flatten)]
    pub flags: Flags,

    #[clap(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Generates shell completions")]
    Completion {
        #[clap(long)]
        shell: Option<Shell>,
    },
}

#[derive(Parser, Debug)]
pub struct Flags {
    #[clap(short, long, help = "Clears the cache")]
    pub clear: bool,

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

    #[clap(short, long, help = "Prints verbose information")]
    pub verbose: bool,
}
