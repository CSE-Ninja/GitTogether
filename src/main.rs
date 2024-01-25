use activity_action::api::Contributor;
use activity_action::api::ContributorExt;
use activity_action::card::draw_svg;
use activity_action::period::parse_from_input;
use activity_action::period::Period;
use activity_action::renderer;
use activity_action::template::construct_table;
// use activity_action::template::construct_table;
use std::env;
use std::fs;
use std::path::Path;

async fn process(repository: &String, periods: Vec<Period>) -> i32 {
    let token = env::var("GITHUB_TOKEN").unwrap();
    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(token)
        .build()
        .unwrap();
    let v: Vec<&str> = repository.split("/").collect();
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
                    sections.push_str(construct_table(&repository, &stat, &period).as_str());
                    data.push((period, stat));
                }
            }
        }
    }

    fs::write("Contributors.md", sections).expect("Failed to write file.");


    svg::save("image.svg", &draw_svg(&data)).unwrap();
    let mut render = renderer::Renderer::new(1960, 1080).unwrap();
    render.render(Path::new("image.svg")).unwrap();

    println!("Contributor list generated successfully.");

    return 0;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let repository = &args[1];
    let periods = parse_from_input(&args[2]);
    std::process::exit(process(repository, periods).await);
}
