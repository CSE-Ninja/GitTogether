use chrono::{DateTime, Duration, Utc};

pub struct Period {
    pub name: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Period {
    // TODO implement fmt::Display for Period
    pub fn to_string(&self) -> String {
        format!(
            "{} ({}–{})",
            self.name,
            self.start.format("%Y-%m-%d"),
            self.end.format("%Y-%m-%d"),
        )
    }

    pub fn from_string(input: &str) -> Vec<Period> {
        input
            .split(';')
            .filter_map(Self::parse_one)
            .collect()
    }

    fn parse_one(spec: &str) -> Option<Period> {
        let mut parts = spec.splitn(3, '/');
        let name = parts.next()?.trim().to_owned();
        let start_s = parts.next()?.trim();
        let end_s = parts.next()?.trim();

        let start = DateTime::parse_from_rfc3339(start_s).ok()?.with_timezone(&Utc);
        let end   = DateTime::parse_from_rfc3339(end_s).ok()?.with_timezone(&Utc);

        Some(Period { name, start, end })
    }

    pub fn last_month() -> Vec<Period> {
        let name = "Last Month".to_string();
        let end = Utc::now();
        let start = end - Duration::days(30);
        vec![Period { name, start, end }]
    }
}
