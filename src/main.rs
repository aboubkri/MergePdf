use clap::Parser;
use std::path::PathBuf;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

// Import from your own library
use mergepdf::pdf::merge::merge_multiple_pdfs;

#[derive(Parser, Debug)]
#[command(author, version, about = "A professional tool for merging PDF files")]
struct Args {
    #[arg(required = true, num_args = 1..)]
    inputs: Vec<PathBuf>,

    #[arg(short, long, default_value = "merged_output.pdf")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let args = Args::parse();

    // Use the core logic directly
    if let Err(e) = merge_multiple_pdfs(&args.inputs, args.output) {
        error!("Merge failed: {}", e);
        std::process::exit(1);
    }

    println!("Successfully merged PDFs!");
    Ok(())
}