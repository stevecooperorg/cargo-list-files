use std::path::PathBuf;

use cargo_list_files;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
    toml: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let files = cargo_list_files::list_files(&cli.toml);
    for file in files {
        println!("{}", file.to_string_lossy());
    }
}
