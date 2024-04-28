mod config;
mod history;
mod parse;
mod request;
use config::*;
use history::*;
use parse::*;
use request::*;
use tokio::time::{sleep, Duration};

fn all_antitag(api_config: &Config) -> String {
    let mut tags: String = "".to_string();
    for antitag in &api_config.antitags {
        tags.push_str("+-");
        tags.push_str(antitag);
    }
    tags
}

#[tokio::main]
async fn main() {
    let config = Config::load();
    let mut history = History::load();
    let anti_tags = all_antitag(&config);

    loop {
        let mut all_post = vec![];

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

            let post = Parse::get_html(&repuest, Parse::get_post()).await;
            for item in post {
                let cleaned_result = item.split("&tags").next().unwrap_or(&item);
                if history.processed_urls.contains(cleaned_result) || all_post.contains(&item) {
                    continue;
                }

                all_post.push(cleaned_result.to_string());

                history.processed_urls.insert(cleaned_result.to_string());
            }
        }

        for item in &all_post {
            let repuest = match request(item).await {
                Ok(response) => response,
                Err(message) => {
                    eprintln!("{}", message);
                    continue;
                }
            };

            let get_image = Parse::get_html(&repuest, Parse::get_image()).await;
            let get_video = Parse::get_html(&repuest, Parse::get_video()).await;

            let content: (&str, bool) = if let Some(e) = get_image.last() {
                (e, false)
            } else if let Some(e) = get_video.first() {
                (e, true)
            } else {
                history.processed_urls.remove(item);
                continue;
            };

            if let Ok(()) = webhook_send(&config.webhook_url, content.0, item, content.1).await {
                println!("Send: {}", item);
            } else {
                println!("Error send: {}", item);
                history.processed_urls.remove(item);
            };

            sleep(Duration::from_secs(2)).await;
        }

        History::save(&history);

        println!("Sleep 30 min");
        sleep(Duration::from_secs(30 * 60)).await;
    }
}
