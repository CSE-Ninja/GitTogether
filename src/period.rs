use chrono::{DateTime, FixedOffset, Utc};

pub struct Period {
    pub name: String,
    pub start: String,
    pub end: String,
}

pub fn parse_from_input(input: &String) -> Vec<Period> {
    input
        .split(";")
        .map(|it| {
            let v = it.split("/").take(3).collect::<Vec<&str>>();
            println!("{}", it);
            let name = v[0].to_string();
            let start = v[1].to_string();
            let end = v[2].to_string();
            // let start = DateTime::parse_from_rfc3339(&v[1]).unwrap();
            // let end = DateTime::parse_from_rfc3339(&v[2]).unwrap();
            Period { name, start, end }
        })
        .collect()
}
