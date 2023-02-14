use serde::Deserialize;
use std::error::Error;

#[derive(thiserror::Errorp, Debug)]
enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed,
    #[error("Failed converting response to string")]
    FailedResponseToString,

}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response: String = ureq::get(url).call().map_err(|e: | NewsApiError::RequestFailed)
    ?.into_string().map_err(|e: | NewsApiError::FailedResponseToString)?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}
