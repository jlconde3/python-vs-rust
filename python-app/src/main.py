import json
from datetime import datetime
from dataclasses import dataclass

@dataclass
class CurrencyExchange:
    """Class/structure to create objects
    based on data from a JSON file."""
    date_of_exchange: datetime
    from_currency: str
    to_currency: str
    exchange_rate: float

def open_json_file(file_path: str) -> dict:
    """Reads a JSON file and returns its content as a dictionary."""
    try:
        with open(file_path, "r") as file:
            file_content = json.load(file)
        return file_content
    except Exception as error:
        print(f"An error occurred while reading the file: {error}")
        raise

def parse_date_of_exchange(date_of_exchange_str: str) -> datetime:
    """Parses the date of the currency exchange."""
    try:
        date_of_exchange = datetime.strptime(date_of_exchange_str, "%Y-%m-%d")
        return date_of_exchange
    except Exception as error:
        print(f"An error occurred while parsing the date of exchange: {error}")
        raise

def parse_json_file(file: dict) -> list[CurrencyExchange]:
    """Parses the service response to create CurrencyExchange objects."""
    try:
        parsed_data = []
        response: dict = file.get("response", {})
        date_of_exchange_str: str = response.get("date", {})  # Currency exchange date
        date_of_exchange: datetime = parse_date_of_exchange(date_of_exchange_str)
        from_currency: str = response.get("base", "")
        exchanges_data: dict = response.get("rates", {})

        for to_currency, exchange_rate in exchanges_data.items():
            currency_exchange = CurrencyExchange(date_of_exchange, from_currency, to_currency, exchange_rate)
            parsed_data.append(currency_exchange)

        return parsed_data
    except Exception as error:
        print(f"There was an error parsing the response: {error}")
        raise

if __name__ == "__main__":
    # Provide the path to the JSON file
    file_path = "/Users/jlconde/source/python-vs-rust/data.json"

    # Open the JSON file and parse its content
    file = open_json_file(file_path)
    data = parse_json_file(file)

    # Display the attributes and values of each CurrencyExchange object
    for exchange in data:
       print(exchange.__dict__)
