extern crate reqwest;

use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define your API endpoint
    let api_endpoint = "http://api.openweathermap.org/data/2.5/weather?q=";

    // Define your city and country code
    let city = "Ankara";
    let country_code = "TR";

    // Define your OpenWeatherMap API key
    let open_weather_map_api_key = "your_openweathermap_api_key";

    // Construct the full URL
    let full_url = format!("{}{},{}&APPID={}", api_endpoint, city, country_code, open_weather_map_api_key);

    // Use the reqwest::get function to make the request
    let mut response = reqwest::get(&full_url)?.text()?;

    // The response will be a JSON string containing weather data
    let weather_data: serde_json::Value = serde_json::from_str(&response)?;

    // You can now use the weather_data object to access the weather information
    println!("Temperature: {}", weather_data["main"]["temp"]);
    println!("Pressure: {}", weather_data["main"]["pressure"]);
    println!("Humidity: {}", weather_data["main"]["humidity"]);
    println!("Wind Speed: {}", weather_data["wind"]["speed"]);

    Ok(())
}