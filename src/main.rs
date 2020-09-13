use anyhow::Result;
use dotenv::dotenv;
use rand::distributions::{Distribution, Uniform};
use serde::Deserialize;
use std::env;
use std::fs;

fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;
    let saved_path = env::var("SAVE_PATH")?;

    let request_url = request_url_builder(api_key.as_str());
    let response = make_request(&request_url)?;

    let (source_name, title, url) = get_data(response);

    fs::write(saved_path, url)?;

    println!(
        "{}: {}",
        source_name.trim_matches('"'),
        title.trim_matches('"')
    );
    Ok(())
}

fn generate_random(range: usize) -> usize {
    let step = Uniform::new(0, range);
    let mut rng = rand::thread_rng();
    let choice = step.sample(&mut rng);
    choice
}

fn request_url_builder(api_key: &str) -> String {
    format!(
        "https://newsapi.org/v2/top-headlines?apiKey={}&country=us",
        api_key
    )
}

fn make_request(url: &str) -> Result<Vec<Article>> {
    let resp = ureq::get(&url).call().into_json_deserialize::<Response>()?;
    Ok(resp.articles)
}

fn get_data(data: Vec<Article>) -> (String, String, String) {
    let random_article = generate_random(data.len());

    let selected_article = &data[random_article];

    let source_name = &selected_article.source.name;
    let title = &selected_article.title;
    let url = &selected_article.url;

    (source_name.into(), title.into(), url.into())
}

#[derive(Deserialize)]
struct Response {
    articles: Vec<Article>,
}

#[derive(Debug, Deserialize)]
struct Article {
    source: Source,
    title: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Source {
    name: String,
}
