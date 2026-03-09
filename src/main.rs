use clap::Parser;
use std::path::PathBuf;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

use mergepdf::pdf::merge::merge_multiple_pdfs;

#[derive(Parser, Debug)]
#[command(author, version, about = "A professional tool for merging PDF files")]
struct Args {
    /// Input PDF files to merge (requires at least one)
    #[arg(required = true, num_args = 1..)]
    inputs: Vec<PathBuf>,

    /// Output PDF file path
    #[arg(short, long, default_value = "merged_output.pdf")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    // Initialize professional logging (replaces println!)
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Parse command line arguments
    let args = Args::parse();

    // Execute core logic and handle errors gracefully
    if let Err(e) = merge_multiple_pdfs(&args.inputs, args.output) {
        error!("Merge failed: {}", e);
        std::process::exit(1);
    }

    Ok(())
}