use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct History {
    pub processed_urls: HashSet<String>,
}

impl History {
    pub fn load() -> History {
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

    pub fn save(history: &History) {
        let history_content =
            serde_json::to_string(history).expect("Error serializing history to JSON");
        fs::write("history.json", history_content).expect("Error writing history file");
    }
}
