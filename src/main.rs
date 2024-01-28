use activity_action::period::{get_recent10_days, parse_from_input};
use activity_action::process;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let repository = &args[1];
    let periods = if args.len() > 2 {
        parse_from_input(&args[2])
    } else {
        get_recent10_days()
    };
    let svg = process(repository, periods).await;
    svg::save("image.svg", &svg).unwrap();
}
