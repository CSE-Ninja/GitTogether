use activity_action::period::{get_recent_one_month, parse_from_input};
use activity_action::process;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    repo: String,
    period: Option<String>,

    #[arg(short, long, default_value = "compact")]
    style: String,
}


#[tokio::main]
async fn main() {
    let args = Args::parse();
    let periods = if let Some(period_str) = args.period {
        parse_from_input(&period_str)
    } else {
        parse_from_input("Project 2(S1)/2024-01-20T00:00:00-05:00/2024-02-13T23:59:00-05:00;Project 2(S2)/2024-02-14T00:00:00-05:00/2024-02-29T23:59:00-05:00;Project 3/2024-03-01T00:00:00-05:00/2024-03-21T23:59:00-05:00")
    };

    let svg = process(&args.repo, periods, &args.style).await;
    svg::save("image.svg", &svg).unwrap();
}
