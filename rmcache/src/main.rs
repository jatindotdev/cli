use clap::Parser;
use color_print::cprintln;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct CLI {}

fn main() {
    let args = CLI::parse();
}
