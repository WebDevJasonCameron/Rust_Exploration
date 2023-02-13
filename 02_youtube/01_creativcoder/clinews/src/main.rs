use colour::{dark_green, yellow};
use newsapi::{get_articles, Articles};
use std::error::Error;

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_green!("> {}\n", a.title);
        yellow!("- {}\n\n", a.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url: &str =
        "https://newsapi.org/v2/everything?q=Apple&from=2023-02-12&sortBy=popularity&apiKey=";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
