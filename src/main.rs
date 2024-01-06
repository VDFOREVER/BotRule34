use serde::{Deserialize, Serialize};
use reqwest::{self};
use std::collections::HashSet;
use std::fs;
use tokio::time::{sleep, Duration};
use reqwest::Error;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, Serialize)]
struct Post {
    #[serde(rename = "file_url")]
    file_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Posts {
    #[serde(rename = "post")]
    posts: Vec<Post>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiConfig {
    url: String,
    webhook_url: String,
    antitags: Vec<String>,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct History {
    processed_urls: HashSet<String>,
}

async fn fetch_xml_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}

fn load_config() -> ApiConfig {
    let config_content = fs::read_to_string("config.json").expect("Error reading config file");
    serde_json::from_str(&config_content).expect("Error parsing config JSON")
}

fn load_history() -> History {
    if let Ok(history_content) = fs::read_to_string("history.json") {
        serde_json::from_str(&history_content).unwrap_or_else(|_| History {
            processed_urls: HashSet::new(),
        })
    } else {
        History {
            processed_urls: HashSet::new(),
        }
    }
}

fn save_history(history: &History) {
    let history_content =
        serde_json::to_string(history).expect("Error serializing history to JSON");
    fs::write("history.json", history_content).expect("Error writing history file");
}

async fn webhook_send(url: &str, content: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let json_str = format!(r#"{{"content": "{}"}}"#, content);

    let _ = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_str)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let api_config = load_config();
    let mut history = load_history();

    loop {
        for tag in &api_config.tags {
            
            let mut full_url = api_config.url.clone();
            full_url.push_str(tag);

            for antitag in &api_config.antitags {
                full_url.push_str("+-");
                full_url.push_str(antitag);
            }
            
            match fetch_xml_data(&full_url).await {
                Ok(xml_data) => {
                    let result: Result<Posts, _> = from_str(&xml_data);
                    match result {
                        Ok(posts) => {
                            println!("{}", full_url);
                            for post in posts.posts {
                                if !history.processed_urls.contains(&post.file_url) {
                                    if let Err(e) = webhook_send(&api_config.webhook_url, &post.file_url).await {
                                        eprintln!("Error sending to webhook: {}", e);
                                    } else {
                                        println!("Sent to webhook: {}", &post.file_url);
                                    }
                                    history.processed_urls.insert(post.file_url.clone());
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Error parsing XML data: {}", err);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error fetching XML data: {}", err);
                }
            }
        }

        save_history(&history);

        println!("Sleep 30 min");
        // Пауза в 30 минут перед следующей проверкой
        sleep(Duration::from_secs(30 * 60)).await;
    }
}
