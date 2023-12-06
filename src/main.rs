use serde::Deserialize;
use serde_xml_rs::from_str;
use reqwest;
use std::fs;

#[derive(Debug, Deserialize)]
struct Post {
    #[serde(rename = "file_url")]
    file_url: String,
}

#[derive(Debug, Deserialize)]
struct Posts {
    #[serde(rename = "post")]
    posts: Vec<Post>,
}

#[derive(Debug, Deserialize)]
struct ApiConfig {
    url: String,
    tags: Vec<String>,
}

async fn fetch_xml_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}


fn load_config() -> ApiConfig {
    let config_content = fs::read_to_string("config.json").expect("Error reading config file");
    serde_json::from_str(&config_content).expect("Error parsing config JSON")
}

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let api_config = load_config();

        for tag in api_config.tags {
            let full_url = format!("{}{}", api_config.url, tag);
            let xml_data = fetch_xml_data(&full_url).await.unwrap();

            let result: Posts = from_str(&xml_data).unwrap();

            for post in result.posts {
                println!("File URL: {}", post.file_url);
            }
        }
    });
}