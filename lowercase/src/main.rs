use std::{fs, path::Path, process::exit};

use clap::Parser;
use color_print::cprintln;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct CLI {
    path: String,
    #[arg(short, long, default_value = " ", help = "File name delimiter.")]
    delimiter: String,
    #[arg(
        short,
        long,
        default_value = "-",
        help = "Delimiter used instead of space."
    )]
    replacer: String,
    #[arg(
        short,
        long = "lo, lowercase-only",
        default_value = "false",
        help = "Whether to lowercase the file name."
    )]
    lowercase_only: bool,
}

fn main() {
    let args = CLI::parse();
    let path = fs::canonicalize(args.path).unwrap_or_else(|_| {
        cprintln!("<red,bold>Error:</> Fail to parse file.");
        exit(1)
    });

    let filepath = path.parent().and_then(|p| p.to_str()).unwrap_or_else(|| {
        cprintln!("<red,bold>Error:</> Fail to parse file.");
        exit(1)
    });

    let filename = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or_else(|| {
            cprintln!("<red,bold>Error:</> Fail to parse file.");
            exit(1)
        });

    let new_filename = {
        let mut new_filename = filename.to_lowercase();

        if !args.lowercase_only {
            new_filename = new_filename.replace(&args.delimiter, &args.replacer);
        }

        new_filename
    };

    if filename == new_filename {
        cprintln!("<yellow>Info:</> File is already lowercase.");
        exit(0);
    }

    let new_filepath = Path::new(filepath).join(&new_filename);

    if let Err(_) = fs::rename(path, new_filepath) {
        cprintln!("<red,bold>Error:</> {}", "Fail to rename file.");
        exit(1);
    }
}
