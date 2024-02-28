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

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    meta: Meta,
    response: Response,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    code: u32,
    disclaimer: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    date: String,
    base: String,
    rates: Rates,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rates {
    USD: f64,
    EUR: f64,
    GBP: f64,
    AUD: f64,
    CAD: f64,
    SGD: f64,
    CHF: f64,
    MYR: f64,
    JPY: f64,
    CNY: f64,
    NZD: f64,
    THB: f64,
    HUF: f64,
    HKD: f64,
    ZAR: f64,
    PHP: f64,
    SEK: f64,
    IDR: f64,
    BRL: f64,
    SAR: f64,
    KRW: f64,
    EGP: f64,
    NOK: f64,
    DKK: f64,
    PKR: f64,
    ILS: f64,
    XAU: f64,
    CLP: f64,
    TWD: f64,
    CZK: f64,
    JOD: f64,
    FJD: f64,
    DZD: f64,
    ISK: f64,
    XAG: f64,
    JMD: f64,
    LBP: f64,
    TTD: f64,
    XCD: f64,
    BBD: f64,
    BMD: f64,
    XPT: f64,
    XDR: f64,
    BSD: f64,
}

fn open_json_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file: File = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_json_file(file_content: &str) -> Result<ApiResponse, serde_json::Error> {
    let parsed_data: Result<ApiResponse, serde_json::Error> = serde_json::from_str(file_content);
    parsed_data
}

fn main() {
    // Provide the path to the JSON file
    let file_path = "/Users/jlconde/source/python-vs-rust/data.json";

    // Open the JSON file and parse its content
    if let Ok(file_content) = open_json_file(&file_path) {
        match parse_json_file(&file_content) {
            Ok(response) => {
                let from_currency = response.response.base;
                let rates = serde_json::to_value(response.response.rates).unwrap();

                // Example: Iterate through rates and print them
                for (to_currency, exchange_rate) in rates.as_object().unwrap() {
                    println!(
                        "From {} to {} => {}",
                        from_currency, to_currency, exchange_rate
                    );
                }
            }
            Err(err) => eprintln!("Error parsing JSON: {}", err),
        }
    } else {
        eprintln!("Error opening JSON file");
    }
}
