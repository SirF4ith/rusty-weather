use std::{error::Error, fmt};
use clap::Parser;
use open_meteo_rs::forecast::{ForecastResult, ForecastResultDaily};
use datetime::{DatePiece, LocalDateTime};
use geocoding::{Openstreetmap, Forward, Point};
use tokio::task;

const TITLE: &str = "rusty-weather";

#[derive(Parser)]
#[command(version = "0.1")]
#[command(about = "A weather forecast CLI tool", long_about = None)]
struct Cli {
    #[arg(long)]
    latitude: f64,
    #[arg(long)]
    longitude: f64,
    #[arg(long)]
    place: Option<String>,
}

struct WeatherData {
    temperature: f64,
    precipitation: f64,
    wind_speed: f64,
    daily: Vec<ForecastResultDaily>,
}

//enum ShorterWeekdays {
//    Sun, Mon, Tue, Wed, Thu, Fri, Sat
//}
//
//impl ShorterWeekdays {
//    fn from_full_weekday(weekday: datetime::Weekday) -> Result<String, Error> {
//        Ok(match weekday {
//            Weekday::Friday => Fri,     Weekday::Monday => Mon,
//            Weekday::Tuesday => Tue,    Weekday::Saturday => Sat,
//            Weekday::Sunday => Sun,     Weekday::Thursday => Thu,
//            Weekday::Wednesday => Wed,  _ => return Err(FormatError),
//        })
//    }
//}

impl fmt::Display for WeatherData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        // weather data
        // current
        let temp_str = format!("{:.1}°C", self.temperature);
        let prec_str = format!("{:.1}mm", self.precipitation);
        let wind_str = format!("{:.2}kmh", self.wind_speed);

        // daily
        let mut temp_max_vec: Vec<f64> = vec![0.0; 7];
        let mut temp_min_vec: Vec<f64> = vec![0.0; 7];
        let mut prec_prob_vec: Vec<f64> = vec![0.0; 7];
        let mut wind_vec: Vec<f64> = vec![0.0; 7];


        for index in 0..self.daily.len() {
            temp_max_vec[index] = self.daily[index].values.get("temperature_2m_max").unwrap().value.as_f64().unwrap();
            temp_min_vec[index] = self.daily[index].values.get("temperature_2m_min").unwrap().value.as_f64().unwrap();
            prec_prob_vec[index] = self.daily[index].values.get("precipitation_probability_max").unwrap().value.as_f64().unwrap();
            wind_vec[index] = self.daily[index].values.get("wind_speed_10m_max").unwrap().value.as_f64().unwrap();
        }

        let temp_max_str: Vec<String> = temp_max_vec.iter().map(|&x| format!("{:.1}", x)).collect();
        

        // datetime
        let datetime = LocalDateTime::now();
        let current_weekday = datetime.date().weekday();


        // ☼, 🌤, 🌦
        let days = vec!["Mon","Tue","Wed","Thu","Fri","Sat","Sun"];

        //box printing
        let width = 9*7 + 2*6 + 4; // Account for padding and borders
        println!("╔{:═^width$}╗", format!(" {} ", TITLE), width = width);
        println!("║{:width$}║","", width = width);
        println!("║{:<width$}║", " Current weather ", width = width);
        println!("║{:<width$}║", format!(" {} {} {} ", &temp_str, &prec_str, &wind_str), width = width);
        println!("║{:width$}║","", width = width);
        println!("╠{:═^width$}╣", "", width = width);
        println!("║{:<width$}║", days.join(" "), width = width);
        println!("║{:<width$}║", temp_max_str.join(" "), width = width);
        println!("╚{:═^width$}╝", "" , width = width);
        
        write!(f, "")
    }
}

fn get_coordinates(place: String) -> [f64;2] {
    
    let osm = Openstreetmap::new();
    let address = place;
    let res = osm.forward(&address).unwrap()[0];
    let x = res.y();
    let y = res.x();

    [x,y]
}

async fn get_weather(coordinates: [f64;2]) -> ForecastResult {
    // using the open_meteo_rs api to fetch the weather data
    let client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();

    let x = coordinates[0];
    let y = coordinates[1];

    opts.location = open_meteo_rs::Location {
        lat: x,
        lng: y,
    };

    opts.current.push("temperature_2m".into());
    opts.current.push("relative_humidity_2m".into());
    opts.current.push("precipitation".into());
    opts.current.push("rain".into());
    opts.current.push("wind_speed_10m".into());

    opts.daily.push("temperature_2m_max".into());
    opts.daily.push("temperature_2m_min".into());
    opts.daily.push("precipitation_sum".into());
    opts.daily.push("precipitation_probability_max".into());
    opts.daily.push("wind_speed_10m_max".into());

    let res = client.forecast(opts).await.unwrap();

    res
}

#[tokio::main]
async fn main() {
    // takes cli-arguments
    let args = Cli::parse();
    
    // Coordinates from args
    let coordinates = [args.latitude, args.longitude];

    // Coordinates from OSM with city name
    let mut coos: [f64;2] = [0.0, 0.0];
    task::block_in_place(|| {
        coos = get_coordinates(args.place.unwrap());
    });
    
    // open_meteo_rs ForecastResult for coordinates
    let res = get_weather(coos).await;

    // daily forecast
    let daily = res.daily.unwrap();

    // current weather
    let current = res.current.unwrap().values;
    let temperature = current.get("temperature_2m").unwrap().value.as_f64().unwrap();
    let precipitation = current.get("precipitation").unwrap().value.as_f64().unwrap();
    let wind_speed = current.get("wind_speed_10m").unwrap().value.as_f64().unwrap();
    
    
    let weather = WeatherData {
        temperature: temperature,
        precipitation: precipitation,
        wind_speed: wind_speed,
        daily: daily,
    };
    println!("{}", weather);
    
}
