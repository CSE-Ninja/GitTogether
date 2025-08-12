use chrono::{Duration, Utc};

pub struct Period {
    pub name: String,
    pub start: String,
    pub end: String,
}

impl Period {
    // TODO implement fmt::Display for Period
    pub fn to_string(&self) -> String {
        format!(
            "{} ({}–{})",
            self.name,
            self.start.get(0..10).unwrap(),
            self.end.get(0..10).unwrap(),
        )
    }

    pub fn from_string(input: &str) -> Vec<Period> {
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

    pub fn last_month() -> Vec<Period> {
        let name = "Last Month".to_string();
        let now = Utc::now();
        let start = (now - Duration::days(30)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let end = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        vec![Period { name, start, end }]
    }
}
