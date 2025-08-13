use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use gittogether::period::{Period};
use gittogether::process;

use clap::Parser;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub periods: Vec<Period>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    repo: String,
    period: Option<String>,

    #[arg(short, long, default_value = "compact")]
    style: String,

    #[arg(short, long)]
    config: Option<PathBuf>,

    #[arg(short, long, default_value = "image.svg")]
    output: PathBuf,
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.config.is_some() && args.period.is_some() {
        eprintln!("warning: --config provided; ignoring positional period string");
    }

    let periods: Vec<Period> = if let Some(cfg_path) = args.config.as_ref() {
        let text = fs::read_to_string(cfg_path)
            .with_context(|| format!("failed to read config file {}", cfg_path.display()))?;
        let cfg: Config = serde_yml::from_str(&text)
            .with_context(|| format!("failed to parse YAML in {}", cfg_path.display()))?;
        cfg.periods
    } else if let Some(period_str) = args.period.as_ref() {
        Period::from_string(period_str)
    } else {
        Period::last_month()
    };

    let svg = process(&args.repo, periods, &args.style).await;
    svg::save(&args.output, &svg)
        .with_context(|| format!("Failed writing output SVG to {}", args.output.display()))?;

    Ok(())
}
