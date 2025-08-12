use std::{path::PathBuf};

use anyhow::{Context, Result};

use gittogether::period::{Period};
use gittogether::process;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    repo: String,
    period: Option<String>,

    #[arg(short, long, default_value = "compact")]
    style: String,

    #[arg(short, long, default_value = "image.svg")]
    output: PathBuf,
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let periods = if let Some(period_str) = args.period {
        Period::from_string(&period_str)
    } else {
        Period::last_month()
    };

    let svg = process(&args.repo, periods, &args.style).await;
    svg::save(&args.output, &svg)
        .with_context(|| format!("Failed writing output SVG to {}", args.output.display()))?;

    Ok(())
}
