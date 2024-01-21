use activity_action::get_contributor_stats::ContributorExt;
use activity_action::template::construct_table;
use core::time;
use std::env;
use std::fs;
use std::thread;

#[tokio::main]
async fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap();

    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];
    let repository = &args[3];
    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(token)
        .build()
        .unwrap();

    // We need retry here because https://stackoverflow.com/questions/56416465/github-api-repos-owner-repo-contributors-returns-an-empty-object
    for _ in 0..10 {
        let result = octocrab
            .list_contributors_stats(repository.to_string())
            .await;
        match result {
            Err(e) => {
                println!("Failed to fetch contributors {}", e);
            }
            Ok(contributors) => {
                let contents =
                    fs::read_to_string(input).expect("Failed to read file.");
                let new_contents =
                    contents.replace("{-ActivityLocation-}", &construct_table(repository.to_string(),contributors));
                fs::write(output, new_contents).expect("Failed to write file.");
                println!("Contributor list generated successfully.");
                std::process::exit(0);
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
    println!("All retries failed.");
    std::process::exit(-1);
}
