use serde::{Deserialize, Deserializer};

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Article parsing failed")]
    ArticleParseFailed(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    #[serde(deserialize_with = "deserialize_null_default")]
    pub title: String,
    pub url: String,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt: Option<T> = Option::deserialize(deserializer)?;

    Ok(opt.unwrap_or_default())
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response: String = ureq::get(url)
        .call()
        .map_err(|e: ureq::Error| NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err(|e: std::io::Error| NewsApiError::FailedResponseToString(e))?;

    dbg!(&response);

    let articles: Articles = serde_json::from_str(&response)
        .map_err(|e: serde_json::Error| NewsApiError::ArticleParseFailed(e))?;

    Ok(articles)
}
