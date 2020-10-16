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

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.weather.iter();
        let mapped = iter.map(|wd| WeatherDataSet::new(wd, wd.source(&self.sources)));
        let folded = mapped
            .fold(None, |a, b| match (a, b) {
                (Some(a), Some(b)) => Some(format!("{}\n\n{}", a, b)),
                (None, Some(b)) => Some(b.to_string()),
                (_, None) => None,
            })
            .unwrap_or_else(|| "[no data]".to_owned());
        write!(f, "{}", folded)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WeatherData {
    #[serde(with = "date_serde")]
    timestamp: DateTime<FixedOffset>,
    source_id: i32,
    precipitation: f32,
    pressure_msl: f32,
    sunshine: f32,
    temperature: f32,
    wind_direction: f32,
    wind_speed: f32,
    cloud_cover: f32,
    dew_point: f32,
    relative_humidity: Option<f32>,
    visibility: f32,
    wind_gust_direction: Option<f32>,
    wind_gust_speed: f32,
    condition: Condition,
    icon: String,
    fallback_source_ids: Option<Value>,
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
    id: i32,
    dwd_station_id: Option<String>,
    observation_type: ObservationType,
    lat: f32,
    lon: f32,
    height: f32,
    station_name: String,
    wmo_station_id: String,
    #[serde(with = "date_serde")]
    first_record: DateTime<FixedOffset>,
    #[serde(with = "date_serde")]
    last_record: DateTime<FixedOffset>,
    distance: f32,
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
