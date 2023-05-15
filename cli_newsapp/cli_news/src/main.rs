use std::fs;

use colour::{dark_green, yellow};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: Option<String>,
    url: Option<String>,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        if a.title != None && a.url != None {
            dark_green!("> {:?}\n", a.title);
            yellow!("- {:?}\n\n", a.url);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _api_url = api();

    let url = "https://newsapi.org/v2/everything?q=tesla&from=2023-04-15&sortBy=publishedAt&apiKey=2e17be1e32034dd4b769cb795f925267";

    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}

fn api() -> String {
    let api = fs::read_to_string("api.txt");

    match api {
        Ok(url) => url,
        _ => String::from("Some error"),
    }
}
