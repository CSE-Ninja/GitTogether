use std::collections::HashMap;

use activity_action::period::{self, DEFAULT_PERIOD};
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
            period::parse_from_input(DEFAULT_PERIOD)
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