use std::path::PathBuf;

use clap::Parser;

/// Program to generate SQL queries from SQL migration files.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The path to the directory to read from. This has to be a folder containing valid .sql files.
    #[arg(short, long)]
    pub input_dir: PathBuf,

    /// The path to the directory where the generated SQL files will be placed.
    #[arg(short, long)]
    pub output_dir: PathBuf,
}
