use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct CurrencyExchange {
    date: String,
    from_currency: String,
    to_currency: String,
    exchange_rate: f64,
}

fn open_json_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_json_file(file_content: &str) {
    let parsed_data: Result<Vec<CurrencyExchange>, _> = serde_json::from_str(file_content);
    if let Err(err) = &parsed_data {
        eprintln!("Error parsing JSON: {}", err);
    }
}

fn main() {
    // Provide the path to the JSON file
    let file_path = "/Users/jlconde/source/python-vs-rust/data.json";

    // Open the JSON file and parse its content
    if let Ok(file_content) = open_json_file(file_path) {
        parse_json_file(&file_content)
    } else {
        eprintln!("Error opening JSON file");
    }
}
