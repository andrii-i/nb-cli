mod notebook;
mod commands;

use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "jupyter-cli")]
#[command(about = "CLI tool for working with Jupyter notebooks", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read and extract notebook content
    Read(commands::read::ReadArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Read(args) => commands::read::execute(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}
