use reqwest::{Error, Response};
use std::time::Duration;

pub async fn fetch_xml_data(url: &str) -> Result<String, Error> {
    reqwest::Client::new()
        .get(url)
        .timeout(Duration::from_secs(20))
        .send()
        .await?
        .text()
        .await
}

pub async fn webhook_send(url: &str, content: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let json_str = format!(r#"{{"content": "{}"}}"#, content);

    client
        .post(url)
        .timeout(Duration::from_secs(20))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_str)
        .send()
        .await
}
