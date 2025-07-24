// src/main.rs
/*
 * Main executable for SmartFactory
 */

use clap::Parser;
use smartfactory::{Result, run};

#[derive(Parser)]
#[command(version, about = "SmartFactory - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
