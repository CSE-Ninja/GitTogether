use activity_action::api::ContributorExt;
use activity_action::period::parse_from_input;
use activity_action::period::Period;
use activity_action::template::construct_table;
// use activity_action::template::construct_table;
use core::time;
use std::env;
use std::fs;
use std::thread;

async fn process(
    input: &String,
    output: &String,
    repository: &String,
    periods: Vec<Period>,
) -> i32 {
    let token = env::var("GITHUB_TOKEN").unwrap();
    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(token)
        .build()
        .unwrap();
    let v: Vec<&str> = repository.split("/").collect();
    let owner = v[0].to_string();
    let repo = v[1].to_string();

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
                if !stat.stats.is_empty() {
                    sections.push_str(format!("## {} ({}-{})\n", period.name, &period.start[..10], &period.end[..10]).as_str());
                    sections.push_str(construct_table(&repository, &stat).as_str());
                }
            }
        }
    }

            let contents =
                fs::read_to_string(input).expect("Failed to read file.");
            let new_contents =
                contents.replace("{-ActivityLocation-}", &sections);
            fs::write(output, new_contents).expect("Failed to write file.");
            println!("Contributor list generated successfully.");

    return 0;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];
    let repository = &args[3];
    let periods = parse_from_input(&args[4]);

    std::process::exit(process(input, output, repository, periods).await);
}
