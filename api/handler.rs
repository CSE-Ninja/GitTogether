use std::collections::HashMap;

use activity_action::period;
use reqwest::Url;

use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let mut params = HashMap::new();
    for (key, value) in url.query_pairs() {
        params.insert(key.into_owned(), value.into_owned());
    }

    if params.contains_key("repo") {

        let repo = params.get("repo").unwrap();
        let periods = if params.contains_key("period") {
            let period_str = params.get("period").unwrap();
            period::parse_from_input(period_str)
        } else {
            period::parse_from_input("Project 2(S1)/2024-01-20T00:00:00-05:00/2024-02-13T23:59:00-05:00;Project 2(S2)/2024-02-14T00:00:00-05:00/2024-02-29T23:59:00-05:00")
            // period::get_recent_one_month()
        };

        let style = if params.contains_key("style") {
            params.get("style").unwrap()
        } else {
            "compact"
        };

        let svg = activity_action::process(repo, periods, style).await;

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "image/svg+xml")
            .header("charset", "utf-8")
            .body(
                svg
                .to_string()
                .into(),
            )?)
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "application/text")
            .body(
                "Please provide repo and period to start".to_string().into()
            )?)
    }

}