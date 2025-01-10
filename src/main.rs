// main.rs
mod cli;
mod weather;
mod geocoding;
mod weather_api;

use cli::Cli;
use weather::WeatherData;
use geocoding::get_coordinates;
use weather_api::get_weather;
use tokio::task;
use clap::Parser;

#[tokio::main]
async fn main() {
    // argument parsing
    let args = Cli::parse();
    let place = args.place.unwrap();

    // Get coordinates
    let coordinates = match task::block_in_place(|| get_coordinates(&place)) {
        Ok(coords) => coords,
        Err(e) => panic!("Error {e}"),
    };

    // Get weather data
    let res = get_weather(coordinates).await;

    // Process the weather data
    let daily = res.daily.unwrap();
    let current = res.current.unwrap().values;
    let temperature = current.get("temperature_2m").unwrap().value.as_f64().unwrap();
    let precipitation = current.get("precipitation").unwrap().value.as_f64().unwrap();
    let wind_speed = current.get("wind_speed_10m").unwrap().value.as_f64().unwrap();

    // Create and display the weather data
    let weather = WeatherData {
        temperature,
        precipitation,
        wind_speed,
        daily,
        coordinates,
        place,
    };
    
    println!("{}", weather);
    
}
