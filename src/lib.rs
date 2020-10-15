#[macro_use]
extern crate serde_derive;

use chrono::{DateTime, FixedOffset};
use serde_json::Value;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Response {
    weather: Vec<WeatherData>,
    sources: Vec<Source>,
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
    relative_humidity: f32,
    visibility: f32,
    wind_gust_direction: f32,
    wind_gust_speed: f32,
    condition: Condition,
    icon: String,
    fallback_source_ids: Option<Value>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Source {
    id: i32,
    dwd_station_id: String,
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
