mod webhook;
mod parse;
mod config;
mod history;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use crate::config::Config;
use serde_json::json;

#[tokio::main]
async fn main() {
    loop {
        if let Ok(config) = config::load_config_from_file("config.json") {
            if let Err(e) = process_config(&config).await {
                eprintln!("{}", e);
            }
        } else {
            eprintln!("Error loading configuration from file");
        }
        println!("Sleep 30 min");
        sleep(Duration::from_secs(1800)).await;
    }
}

async fn process_config(config: &Config) -> Result<(), String> {
    for page in &config.pages {
        let history_file_path = format!("history_{}.json", page);
        let mut processed_links = history::read_history(&history_file_path).unwrap_or_else(|_| HashMap::new());
        let post_results = parse::parse_html(&page, &config.post.class, &config.post.descendant, &config.post.attr).await.map_err(|e| format!("Error while parsing HTML: {}", e))?;

        if processed_links.is_empty() {
            for result in &post_results {
                processed_links.insert(result.clone(), Vec::new());
            }
        }

        process_post_results(&config, &config.webhook_url, &mut processed_links, &history_file_path, &post_results).await?;
    }
    Ok(())
}

async fn process_post_results(config: &Config, webhook_url: &String, processed_links: &mut HashMap<String, Vec<String>>, history_file_path: &str, post_results: &Vec<String>) -> Result<(), String> {
    println!("Parse post");
    for result in post_results {
        if processed_links.contains_key(result) {
            continue;
        }
        let image_results = parse::parse_html(&result, &config.image.class, &config.image.descendant, &config.image.attr).await.map_err(|e| format!("Error while parsing HTML: {}", e))?;
        process_image_result(config, webhook_url, &image_results, result).await;
        sleep(Duration::from_millis(2000)).await;
        processed_links.insert(result.to_string(), image_results);
    }
    history::write_history(history_file_path, &processed_links).map_err(|e| format!("Error writing history: {}", e))?;
    Ok(())
}

async fn process_image_result(config: &Config, webhook_url: &String, image_results: &Vec<String>, post: &str) {
    println!("Parse Image");
    for result in image_results {
        let author_results = match parse::parse_text(post, &config.author.class, &config.author.descendant).await {
            Ok(res) => res.join(", "),
            Err(e) => format!("Error while parsing author: {}", e),
        };
        let character_results = match parse::parse_text(post, &config.character.class, &config.character.descendant).await {
            Ok(res) => res.join(", "),
            Err(e) => format!("Error while parsing character: {}", e),
        };

        let json_data = json!({
            "embeds": [
                {
                    "title": character_results,
                    "url": post,
                    "author": {
                        "name": author_results
                    },
                    "image": {
                        "url": result
                    }
                }
            ]
        });

        if let Err(e) = webhook::webhook_send(webhook_url, json_data).await {
            eprintln!("Error: {}", e);
        }
        sleep(Duration::from_millis(2000)).await;
    }
}