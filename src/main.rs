mod commands;
mod dash;
mod decrypter;
mod download;
mod hls;
mod merger;
mod progress;
mod subtitles;
mod utils;

use clap::Parser;
use commands::{Args, Commands};
use kdam::term::Colorizer;

fn run() -> anyhow::Result<()> {
    match Args::parse().command {
        Commands::Capture(args) => args.perform()?,
        Commands::Collect(args) => args.perform()?,
        Commands::Decrypt(args) => args.perform()?,
        Commands::Extract(args) => args.perform()?,
        Commands::Merge(args) => args.perform()?,
        Commands::Save(args) => args.to_download_state()?.perform()?,
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}: {}", "error".colorize("bold red"), e);
        std::process::exit(1);
    }
}
