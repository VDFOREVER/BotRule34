use reqwest::Client;
use serde_json::json;
use std::time::Duration;

pub async fn request(url: &str) -> Result<String, Box<reqwest::Error>> {
    let result = Client::new()
        .get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await?
        .text()
        .await?;

    Ok(result)
}

pub async fn webhook_send(
    url: &str,
    content: &str,
    autor: &str,
    is_video: bool,
) -> Result<(), reqwest::Error> {
    let json_str = if !is_video {
        json!({
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
        })
    } else {
        json!({
            "content": content
        })
    };

    Client::new()
        .post(url)
        .timeout(Duration::from_secs(10))
        .json(&json_str)
        .send()
        .await?;

    Ok(())
}
