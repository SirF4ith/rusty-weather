use std::fmt;
use clap::Parser;
use open_meteo_rs::forecast::ForecastResult;


#[derive(Parser)]
struct Cli {
    latitude: f64,
    longitude: f64,
}

struct WeatherData {
    temperature: f64,
    percipitation: f64,
}

impl fmt::Display for WeatherData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let temp_str = format!("{:.1}°C", self.temperature);

        let width = temp_str.len() + 4; // Account for padding and borders
        let border = "╔".to_owned() + &"═".repeat(width - 1) + "╗";
        let middle = "║  ".to_string() + &temp_str + "  ║";
        let bottom = "╚".to_owned() + &"═".repeat(width - 1) + "╝";

        write!(f, "\n{}\n{}\n{}", border, middle, bottom)
    }
}

async fn get_weather(coordinates: [f64;2]) -> ForecastResult {
    let client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();

    let x = coordinates[0];
    let y = coordinates[1];

    opts.location = open_meteo_rs::Location {
        lat: x,
        lng: y,
    };

    opts.current.push("temperature_2m".into());

    let res = client.forecast(opts).await.unwrap();

    res
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    
    let coordinates = [args.latitude, args.longitude];

    let res = get_weather(coordinates).await;

    let current = res.current.unwrap().values;
    let temperature = current.get("temperature_2m").unwrap().value.as_f64().unwrap();
    let weather = WeatherData {
        temperature: temperature,
        percipitation: 0.0,
    };
    println!("{}", weather);
    
}
