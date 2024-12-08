use std::fs::File;
use std::io::{Write, Read};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

pub fn get_input(day: &str) {
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let file_path = format!("/var/data/input_{}.txt", day);

    let session = std::env::var("SESSION").unwrap();

    let mut headers = HeaderMap::new();
    let cookie_value = format!("session={}", session);

    headers.insert(COOKIE, HeaderValue::from_str(&cookie_value).unwrap());

    let client = Client::new();
    let response = client
        .get(&url)
        .headers(headers)
        .send();
    if let Ok(resp) = response {
        let text = resp.text().unwrap();
        let mut file = File::create(file_path).unwrap();
        file.write_all(text.as_bytes()).unwrap();
    } else {
        eprintln!("Failed to get input");
    }
}

pub fn read_input(day: &str) -> String {
    let file_path = format!("/var/data/input_{}.txt", day);
    let mut file = File::open(file_path).unwrap();
    let mut input = String::new();
    file.read_to_string( &mut input).unwrap();
    input
}
