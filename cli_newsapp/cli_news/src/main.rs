use colour::{dark_green, yellow};
use dotenv::dotenv;
use newsapi::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key: String = std::env::var("API_KEY")?;

    let api_url: &str =
        "https://newsapi.org/v2/everything?q=tesla&from=2023-04-15&sortBy=publishedAt&apiKey=";

    let url: String = format!("{}{}", api_url, api_key);

    let articles: Articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_green!("> {}\n", a.title);
        yellow!("- {}\n\n", a.url);
    }
}
