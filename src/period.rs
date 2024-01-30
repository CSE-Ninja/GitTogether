use chrono::{Duration, Utc};

pub struct Period {
    pub name: String,
    pub start: String,
    pub end: String,
}

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


pub fn get_recent10_days() -> Vec<Period> {
    let name = "Recent 10 Days".to_string();
    let now = Utc::now();
    let start = (now - Duration::days(10)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let end = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    vec![Period{name, start, end}]
}