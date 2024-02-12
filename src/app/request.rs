use reqwest::{Client, Error, Response};
use serde_json::json;
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

pub async fn webhook_send(url: &str, content: &str, autor: &str) -> Result<Response, Error> {
    let json_str = json!({
        "embeds": [
            {
                "title": autor,
                "author": {
                    "name": "rule34"
                },
                "image": {
                    "url": content
                }
            }
        ]
    });

    Client::new()
        .post(url)
        .timeout(Duration::from_secs(20))
        .json(&json_str)
        .send()
        .await
}
