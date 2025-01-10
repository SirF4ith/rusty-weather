use std::fmt;
use open_meteo_rs::forecast::ForecastResultDaily;
use chrono::{Datelike, Local};

pub struct WeatherData {
    pub temperature: f64,
    pub precipitation: f64,
    pub wind_speed: f64,
    pub daily: Vec<ForecastResultDaily>,
    pub coordinates: (f64, f64),
    pub place: String,
}

impl fmt::Display for WeatherData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let temp_str = format!("{:.1}°C", self.temperature);
        let prec_str = format!("{:.1}mm", self.precipitation);
        let wind_str = format!("{:.2}kmh", self.wind_speed);

        let mut temp_max_vec: Vec<f64> = vec![0.0; 7];
        let mut temp_min_vec: Vec<f64> = vec![0.0; 7];
        let mut prec_prob_vec: Vec<f64> = vec![0.0; 7];
        let mut wind_vel_vec: Vec<f64> = vec![0.0; 7];

        for index in 0..self.daily.len() {
            temp_max_vec[index] = self.daily[index].values.get("temperature_2m_max").unwrap().value.as_f64().unwrap();
            temp_min_vec[index] = self.daily[index].values.get("temperature_2m_min").unwrap().value.as_f64().unwrap();
            prec_prob_vec[index] = self.daily[index].values.get("precipitation_probability_max").unwrap().value.as_f64().unwrap();
            wind_vel_vec[index] = self.daily[index].values.get("wind_speed_10m_max").unwrap().value.as_f64().unwrap();
        }

        let temp_max_str: Vec<String> = temp_max_vec.iter().map(|&x|   format!(" {:>5.1}°C", x)).collect();
        let temp_min_str: Vec<String> = temp_min_vec.iter().map(|&x|   format!(" {:>5.1}°C", x)).collect();
        let prec_prob_str: Vec<String> = prec_prob_vec.iter().map(|&x| format!("  {:>5.1}%", x)).collect();
        let wind_vel_str: Vec<String> = wind_vel_vec.iter().map(|&x|   format!("{:>5.1}kmh", x)).collect();


        let today = Local::now().naive_local();
        let current_weekday = today.weekday();

        let weekdays = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

        let mut days_of_week = Vec::new();

        for i in 0..7 {
            let index = (current_weekday.num_days_from_monday() + i) % 7;
            let index_us: usize = usize::try_from(index).unwrap();
            days_of_week.push(weekdays[index_us]);
        }

        let width = 9 * 7 + 2 * 6 + 4;

        writeln!(f, "╔{:═^width$}╗", " Rusty Weather ", width = width)?;
        writeln!(f, "║{:width$}║", "", width = width)?;
        writeln!(f, "║{:<width$}║", format!(" {}                     {:?}", "Current weather", self.place), width = width)?;
        writeln!(f, "║{:<width$}║", format!(" {} {} {}             {:?}", temp_str, prec_str, wind_str, self.coordinates), width = width)?;
        writeln!(f, "║{:width$}║", "", width = width)?;
        writeln!(f, "╠{:═^width$}╣", " forecast ", width = width)?;
        writeln!(f, "║{:<width$}║", format!("            {}", days_of_week.join("      ")), width = width)?;
        writeln!(f, "║{:<width$}║", format!("Max:   {}", temp_max_str.join(" ")), width = width)?;
        writeln!(f, "║{:<width$}║", format!("Min:   {}", temp_min_str.join(" ")), width = width)?;
        writeln!(f, "║{:<width$}║", format!("Pre:   {}", prec_prob_str.join(" ")), width = width)?;
        writeln!(f, "║{:<width$}║", format!("Win:   {}", wind_vel_str.join(" ")), width = width)?;
        writeln!(f, "╚{:═^width$}╝", "", width = width)?;
        write!(f, "")

    }
}