use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use ureq;
use rand::distributions::{Distribution, Uniform};


fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;
    let saved_path = env::var("SAVE_PATH")?;

    let request_url = request_url_builder(api_key.as_str());
    let response = make_request(request_url)?;


    let (source_name, title, url) = get_data(response);

    save_url(saved_path, url)?;
    output_text(source_name, title);


    Ok(())
}

fn generate_random() -> usize {
    let step = Uniform::new(0, 20);
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

fn make_request(url: String) -> Result<ureq::SerdeValue, io::Error> {
    let resp = ureq::get(&url).call().into_json()?;
    Ok(resp)
}

fn get_data(data: ureq::SerdeValue) -> (String, String, String) {
    let random_article = generate_random();
    let source_name = data["articles"][random_article]["source"]["name"].to_string();
    let title = data["articles"][random_article]["title"].to_string();
    let url = data["articles"][random_article]["url"].to_string();
    (source_name, title, url)
}

fn save_url(saved_path: String, url: String) -> Result<()> {
    let mut f = File::create(saved_path)?;
    f.write_all(url.as_bytes())?;
    Ok(())
}

fn output_text(source_name: String, title: String) {
    println!(
        "{}: {}",
        source_name.trim_matches('"'),
        title.trim_matches('"')
    );
}
