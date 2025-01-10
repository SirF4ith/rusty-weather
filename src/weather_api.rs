// weather_api.rs
use open_meteo_rs::forecast::ForecastResult;
use open_meteo_rs::Client;
use open_meteo_rs::forecast::Options;

pub async fn get_weather(coordinates: (f64, f64)) -> ForecastResult {
    let client = Client::new();
    let mut opts = Options::default();
    let (x,y) = coordinates;


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

    client.forecast(opts).await.unwrap()
}
