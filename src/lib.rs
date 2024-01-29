use std::{env};

use period::Period;
use styles::get_style;
use svg::Document;

use crate::{api::{Contributor, ContributorExt}, card::draw_svg, template::construct_table};

pub mod api;
pub mod card;
pub mod period;
pub mod template;
pub mod styles;

pub async fn process(repository: &String, periods: Vec<Period>) -> Document {
    let token = env::var("GITHUB_TOKEN").unwrap();
    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(token)
        .build()
        .unwrap();
    let v: Vec<&str> = repository.split('/').collect();
    let owner = v[0].to_string();
    let repo = v[1].to_string();

    let mut data = Vec::<(Period, Vec<Contributor>)>::new();
    let mut sections = String::new();
    for period in periods {
        match octocrab
            .list_contributors_stats(&owner, &repo, &period.start, &period.end)
            .await
        {
            Err(e) => {
                println!("Error: {}", e);
            }
            Ok(stat) => {
                if !stat.is_empty() {
                    sections.push_str(
                        format!(
                            "## {} ({}-{})\n",
                            period.name,
                            &period.start[..10],
                            &period.end[..10]
                        )
                        .as_str(),
                    );
                    sections.push_str(construct_table(repository, &stat, &period).as_str());
                    data.push((period, stat));
                }
            }
        }
    }
    draw_svg(&data, repository, get_style("compact")).await
}