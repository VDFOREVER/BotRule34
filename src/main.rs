mod config;
mod history;
mod request;

use config::*;
use history::*;
use request::*;
use serde::Deserialize;
use serde_xml_rs::from_str;
use tokio::time::{sleep, Duration};

#[derive(Deserialize)]
struct Post {
    file_url: String,
}

#[derive(Deserialize)]
struct Posts {
    post: Vec<Post>,
}

fn all_antitag(api_config: &Config) -> String {
    let mut tags: String = "".to_string();
    for antitag in &api_config.antitags {
        tags.push_str("+-");
        tags.push_str(antitag);
    }
    tags
}

fn is_video(post: &str) -> bool {
    if let Some(extension) = post.split('.').last() {
        return extension == "mp4";
    }
    false
}

#[tokio::main]
async fn main() {
    let config = Config::load();
    let mut history = History::load();
    let anti_tags = all_antitag(&config);

    loop {
        for tag in &config.tags {
            let mut full_url = config.url.clone();
            full_url.push_str(tag);
            full_url.push_str(&anti_tags);

            let repuest = match request(&full_url).await {
                Ok(response) => response,
                Err(message) => {
                    eprintln!("{}", message);
                    continue;
                }
            };

            let result: Result<Posts, _> = from_str(&repuest);
            let pasre = match result {
                Ok(result) => result,
                Err(message) => {
                    eprintln!("{}", message);
                    continue;
                }
            };

            let mut all_post: Vec<String> = vec![];
            for post in pasre.post {
                if history.processed_urls.contains(&post.file_url)
                    || all_post.contains(&post.file_url)
                {
                    continue;
                }

                all_post.push(post.file_url.clone());

                history.processed_urls.insert(post.file_url);
            }

            for post in all_post {
                let is_video = is_video(&post);

                if let Ok(()) = webhook_send(&config.webhook_url, &post, tag, is_video).await {
                    println!("Send: {}", post);
                } else {
                    println!("Error send: {}", post);
                    history.processed_urls.remove(&post);
                };

                sleep(Duration::from_secs(2)).await;
            }
        }

        History::save(&history);

        println!("Sleep 30 min");
        sleep(Duration::from_secs(30 * 60)).await;
    }
}
