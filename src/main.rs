use activity_action::get_contributor_stats::{ContributorExt, ContributorStat};
use build_html::{Html, Table};
use std::env;
use std::fs;

fn build_contribution_table(state: Vec<ContributorStat>) -> String {
    let mut source = Vec::<Vec<String>>::new();
    for contributor in state {
        let mut row = Vec::<String>::new();
        row.push(contributor.author.login);
        // row.push(contributor.author.avatar_url.to_string());
        let img_row = format!("<img src=\"{}\"  alt=\"1\" width = 100px height = 100px >", contributor.author.avatar_url);
        row.push(img_row);

        let mut commit = 0;
        let mut add = 0;
        let mut del = 0;
        for week in contributor.weeks {
            commit += week.c;
            add += week.a;
            del += week.d;
        }

        row.push(commit.to_string());
        row.push(add.to_string());
        row.push(del.to_string());
        source.push(row.clone());
        println!("{:?}", row.join(", "));
    }
    Table::from(source)
        .with_header_row(["Name", "Avatar", "Commit", "Addition", "Deletion"])
        .to_html_string()
}

#[tokio::main]
async fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap();

    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];
    let repository = &args[3];
    // let org = &args[4];

    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(token)
        .build()
        .unwrap();
    // let issues = octocrab.issues("rohanpadhye", "JQF");
    let result = octocrab
        .list_contributors_stats(repository.to_string())
        .await;
    match result {
        Err(e) => {
            println!("Failed to fetch contributors {}", e);
            std::process::exit(-1);
        }
        Ok(contributors) => {
            let contents =
                fs::read_to_string(input).expect("Something went wrong reading the file");
            let new_contents = contents.replace(
                "{-ActivityLocation-}",
                &build_contribution_table(contributors),
            );
            fs::write(output, new_contents).expect("Something went wrong writing the file");
        }
    }
}
