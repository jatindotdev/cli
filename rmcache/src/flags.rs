use clap::Parser;

#[derive(Parser, Debug)]
pub struct Flags {
    #[clap(short, long, help = "Prints verbose information")]
    pub verbose: bool,
}
