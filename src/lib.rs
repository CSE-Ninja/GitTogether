use std::env;

use period::Period;
use styles::get_style;
use svg::Document;

use crate::{
    activity::Activity,
    card::draw_svg,
};

pub mod activity;
pub mod api;
pub mod card;
pub mod period;
pub mod template;
pub mod styles;

pub async fn process(
    repository: &String,
    periods: Vec<Period>,
    style: &str,
) -> Document {
    let token = env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN environment variable is not set");

    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(token)
        .build()
        .expect("Failed to create Octocrab client");

    let v: Vec<&str> = repository.split('/').collect();
    let owner = v[0].to_string();
    let repo = v[1].to_string();

    let activities = Activity::for_periods(&octocrab, &owner, &repo, periods)
        .await
        .expect("Failed to fetch activities");

    draw_svg(&activities, repository, get_style(style).as_ref()).await
}
