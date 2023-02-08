// main.rs
extern crate reqwest;
extern crate serde;
// extern crate serde_json;

use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    description: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct News {
    articles: Vec<Article>,
}

async fn get_news(client: &Client, query: &str) -> Result<News, reqwest::Error> {
    let url = format!("https://newsapi.org/v2/everything?q={}&apiKey=469f68e749ff40928d9a6f0d63565de9", query);
    let resp = client.get(&url).send().await?.json::<News>().await?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: news <query>");
        return Ok(());
    }
    let query = &args[1];
    let client = Client::new();
    let news = get_news(&client, query).await?;
    for article in news.articles {
        println!("{}", article.title);
        println!("{}", article.description);
        println!("{}", article.url);
        println!("");
    }
    Ok(())
}
