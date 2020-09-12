use std::fs::File;
use std::io;
use std::io::Write;
use std::process;

fn main() {
    let api_key: &str = "";
    let saved_path: &str = "/home/sachin/.config/polybar/poly/current_url.txt";

    let request_url = request_url_builder(api_key);
    let resp = make_request(request_url);

    match resp {
        Ok(r) => {
            let source_name = &r["articles"][0]["source"]["name"].to_string();
            let title = &r["articles"][0]["title"].to_string();
            let url = &r["articles"][0]["url"].to_string();

            let mut f = File::create(saved_path).unwrap_or_else(|e| {
                println!("{}", e);
                process::exit(1);
            });

            f.write_all(url.as_bytes()).unwrap_or_else(|e| {
                println!("{}", e);
                process::exit(1);
            });

            println!("{}: {}", source_name.trim_matches('"'), title.trim_matches('"'));
        }
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn request_url_builder(api_key: &str) -> String {
    format!(
        "https://newsapi.org/v2/top-headlines?apiKey={}&sources=bloomberg",
        api_key
    )
}

fn make_request(url: String) -> Result<ureq::SerdeValue, io::Error> {
    let resp = ureq::get(&url).call().into_json();
    resp
}
