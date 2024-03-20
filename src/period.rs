use chrono::{Duration, Utc};

pub struct Period {
    pub name: String,
    pub start: String,
    pub end: String,
}

pub const DEFAULT_PERIOD : &str = "Project 2(S1)/2024-01-20T00:00:00-05:00/2024-02-13T23:59:00-05:00;Project 2(S2)/2024-02-14T00:00:00-05:00/2024-02-29T23:59:00-05:00;Project 3/2024-03-01T00:00:00-05:00/2024-03-21T23:59:00-05:00;Project 4/2024-03-22T00:00:00-05:00/2024-04-05T23:59:00-05:00;Project 5/2024-04-06T00:00:00-05:00/2024-05-07T23:59:00-05:00";

pub fn parse_from_input(input: &str) -> Vec<Period> {
    input
        .split(';')
        .map(|it| {
            let v = it.split('/').take(3).collect::<Vec<&str>>();
            println!("{}", it);
            let name = v[0].to_string();
            let start = v[1].to_string();
            let end = v[2].to_string();
            Period { name, start, end }
        })
        .collect()
}


pub fn get_recent_one_month() -> Vec<Period> {
    let name = "Recent One Month".to_string();
    let now = Utc::now();
    let start = (now - Duration::days(30)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let end = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    vec![Period{name, start, end}]
}
