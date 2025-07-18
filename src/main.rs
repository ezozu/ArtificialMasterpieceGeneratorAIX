// src/main.rs
/*
 * Main executable for ArtificialMasterpieceGeneratorAIX
 */

use clap::Parser;
use artificialmasterpiecegeneratoraix::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialMasterpieceGeneratorAIX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
