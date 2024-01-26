use activity_action::api::Contributor;
use activity_action::api::ContributorExt;
use activity_action::card::draw_svg;
use activity_action::period::parse_from_input;
use activity_action::period::Period;
use activity_action::process;
use activity_action::template::construct_table;
use svg::Document;
// use activity_action::template::construct_table;
use std::env;
use std::fs;
use std::path::Path;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let repository = &args[1];
    let periods = parse_from_input(&args[2]);
    let svg = process(repository, periods).await;
    svg::save("image.svg", &svg).unwrap();
}
