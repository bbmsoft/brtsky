#[macro_use]
extern crate serde_derive;

use chrono::{DateTime, FixedOffset};
use serde_json::Value;
use std::fmt;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Response {
    weather: Vec<WeatherData>,
    sources: Vec<Source>,
}

impl Response {
    pub fn weather_data(&self) -> &Vec<WeatherData> {
        &self.weather
    }

    pub fn sources(&self) -> &Vec<Source> {
        &self.sources
    }

    pub fn weather_data_sets(&self) -> impl Iterator<Item = WeatherDataSet> {
        let iter = self.weather.iter();
        let mapped = iter.filter_map(move |wd| WeatherDataSet::new(wd, wd.source(&self.sources)));
        mapped
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_sets = self.weather_data_sets();
        let folded = data_sets
            .fold(None, |a, b| match a {
                Some(a) => Some(format!("{}\n\n{}", a, b)),
                None => Some(b.to_string()),
            })
            .unwrap_or_else(|| "[no data]".to_owned());
        write!(f, "{}", folded)
    }
}

impl std::str::FromStr for Response {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WeatherData {
    #[serde(with = "date_serde")]
    pub timestamp: DateTime<FixedOffset>,
    pub source_id: i32,
    pub precipitation: f32,
    pub pressure_msl: f32,
    pub sunshine: f32,
    pub temperature: f32,
    pub wind_direction: f32,
    pub wind_speed: f32,
    pub cloud_cover: f32,
    pub dew_point: f32,
    pub relative_humidity: Option<f32>,
    pub visibility: f32,
    pub wind_gust_direction: Option<f32>,
    pub wind_gust_speed: f32,
    pub condition: Condition,
    pub icon: String,
    pub fallback_source_ids: Option<Value>,
}

impl WeatherData {
    fn source<'a>(&self, sources: &'a Vec<Source>) -> Option<&'a Source> {
        let time = &self.timestamp;
        for source in sources {
            if source.contains(time) {
                return Some(source);
            }
        }
        None
    }
}

pub struct WeatherDataSet<'a> {
    weather_data: &'a WeatherData,
    source: &'a Source,
}

impl<'a> WeatherDataSet<'a> {
    fn new(
        weather_data: &'a WeatherData,
        source: Option<&'a Source>,
    ) -> Option<WeatherDataSet<'a>> {
        source.map(|source| WeatherDataSet {
            weather_data,
            source,
        })
    }

    pub fn weather_data(&self) -> &WeatherData {
        &self.weather_data
    }

    pub fn source(&self) -> &Source {
        &self.source
    }
}

impl<'a> fmt::Display for WeatherDataSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "City: {}
Date: {}
Type: {}
Condition: {}
Temperature: {} °C
Sunshine: {} min
Precipitation: {} mm
Wind Speed: {} km/h
Wind Gust Speed: {} km/h
Cloud Cover: {} %
Humidity: {}",
            self.source.station_name,
            self.weather_data.timestamp,
            self.source.observation_type,
            self.weather_data.condition,
            self.weather_data.temperature,
            self.weather_data.sunshine,
            self.weather_data.precipitation,
            self.weather_data.wind_speed,
            self.weather_data.wind_gust_speed,
            self.weather_data.cloud_cover,
            self.weather_data
                .relative_humidity
                .map_or_else(|| "-".to_owned(), |f| format!("{} %", f))
        )
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Source {
    pub id: i32,
    pub dwd_station_id: Option<String>,
    pub observation_type: ObservationType,
    pub lat: f32,
    pub lon: f32,
    pub height: f32,
    pub station_name: String,
    pub wmo_station_id: String,
    #[serde(with = "date_serde")]
    pub first_record: DateTime<FixedOffset>,
    #[serde(with = "date_serde")]
    pub last_record: DateTime<FixedOffset>,
    pub distance: f32,
}

impl Source {
    fn contains(&self, time: &DateTime<FixedOffset>) -> bool {
        &self.first_record <= time && time <= &self.last_record
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum Condition {
    #[serde(rename = "dry")]
    Dry,
    #[serde(rename = "fog")]
    Fog,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "sleet")]
    Sleet,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "hail")]
    Hail,
    #[serde(rename = "thunderstorm")]
    Thunderstorm,
    #[serde(rename = "null")]
    Null,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Dry => write!(f, "Dry"),
            Condition::Fog => write!(f, "Fog"),
            Condition::Rain => write!(f, "Rain"),
            Condition::Sleet => write!(f, "Sleet"),
            Condition::Snow => write!(f, "Snow"),
            Condition::Hail => write!(f, "Hail"),
            Condition::Thunderstorm => write!(f, "Thunder Storm"),
            Condition::Null => write!(f, "-"),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum Icon {
    #[serde(rename = "clear-day")]
    ClearDay,
    #[serde(rename = "clear-night")]
    ClearNight,
    #[serde(rename = "partly-cloudy-day")]
    PartlyCloudyDay,
    #[serde(rename = "partly-cloudy-night")]
    PartlyCloudyNight,
    #[serde(rename = "cloudy")]
    Cloudy,
    #[serde(rename = "fog")]
    Fog,
    #[serde(rename = "wind")]
    Wind,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "sleet")]
    Sleet,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "hail")]
    Hail,
    #[serde(rename = "thunderstorm")]
    Thunderstorm,
    #[serde(rename = "null")]
    Null,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum ObservationType {
    #[serde(rename = "forecast")]
    Forecast,
    #[serde(rename = "synop")]
    Synop,
    #[serde(rename = "current")]
    Current,
    #[serde(rename = "recent")]
    Recent,
    #[serde(rename = "historical")]
    Historical,
}

impl fmt::Display for ObservationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObservationType::Forecast => write!(f, "Forecast"),
            ObservationType::Synop => write!(f, "SYNOP"),
            ObservationType::Current => write!(f, "Current"),
            ObservationType::Recent => write!(f, "Recent"),
            ObservationType::Historical => write!(f, "Historical"),
        }
    }
}

mod date_serde {
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let data = std::fs::read("test/test.json").unwrap();
        let data: Response = serde_json::from_slice(&data).unwrap();
        let timestamp = data.weather[0].timestamp;
        assert_eq!(timestamp.timestamp(), 1587427200);
    }
}
