use serde::Deserialize;
use serde_xml_rs::from_str;
use tokio::time::{sleep, Duration};
mod app;
use crate::app::{config::*, history::*, request::*};

#[derive(Deserialize)]
struct Post {
    #[serde(rename = "file_url")]
    file_url: String,
}

#[derive(Deserialize)]
struct Posts {
    #[serde(rename = "post")]
    posts: Vec<Post>,
}

fn all_antitag(api_config: &ApiConfig) -> String {
    let mut tags: String = "".to_string();
    for antitag in &api_config.antitags {
        tags.push_str("+-");
        tags.push_str(antitag);
    }
    tags
}

#[tokio::main]
async fn main() {
    let api_config = load_config();
    let mut history = load_history();
    let anti_tags = all_antitag(&api_config);

    loop {
        for tag in &api_config.tags {
            let mut full_url = api_config.url.clone();
            full_url.push_str(tag);
            full_url.push_str(&anti_tags);

            match fetch_xml_data(&full_url).await {
                Ok(xml_data) => {
                    let result: Result<Posts, _> = from_str(&xml_data);
                    match result {
                        Ok(posts) => {
                            println!("{}", full_url);
                            for post in posts.posts {
                                if history.processed_urls.contains(&post.file_url) {
                                    continue;
                                }

                                if let Err(e) =
                                    webhook_send(&api_config.webhook_url, &post.file_url, tag).await
                                {
                                    eprintln!("Error sending to webhook: {}", e);
                                } else {
                                    println!("Sent to webhook: {}", &post.file_url);
                                }

                                history.processed_urls.insert(post.file_url);
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
        sleep(Duration::from_secs(30 * 60)).await;
    }
}
