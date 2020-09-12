use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use ureq;

fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;
    let saved_path = env::var("SAVE_PATH")?;

    let request_url = request_url_builder(api_key.as_str());
    let response = make_request(request_url)?;

    let (source_name, title, url) = get_data(response);

    save_url(saved_path,url)?;
    output_text(source_name, title);
    
    Ok(())
}

fn request_url_builder(api_key: &str) -> String {
    format!(
        "https://newsapi.org/v2/top-headlines?apiKey={}&sources=bloomberg",
        api_key
    )
}

fn make_request(url: String) -> Result<ureq::SerdeValue, io::Error> {
    let resp = ureq::get(&url).call().into_json()?;
    Ok(resp)
}

fn get_data(data: ureq::SerdeValue) -> (String, String, String)  {
    let source_name = data["articles"][0]["source"]["name"].to_string();
    let title = data["articles"][0]["title"].to_string();
    let url = data["articles"][0]["url"].to_string();
    (source_name, title, url)
}

fn save_url(saved_path:String, url:String) -> Result<()> {
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