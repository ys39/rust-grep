use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;

#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    // Open the file
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?; // Iterate over the lines of the file
    find_matches(&content, &args.pattern, &mut std::io::stdout())?;
    Ok(())
}
