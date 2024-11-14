// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::info_board;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: info_board = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize,Debug)]
pub struct InfoBoard {
    pub location: Location,

    pub current: Current,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Current {
    last_updated_epoch: i64,

    last_updated: String,

    pub temp_c: f64,

    temp_f: f64,

    is_day: i64,

    condition: Condition,

    wind_mph: f64,

    wind_kph: f64,

    wind_degree: i64,

    wind_dir: String,

    pressure_mb: f64,

    pressure_in: f64,

    precip_mm: f64,

    precip_in: f64,

    humidity: i64,

    cloud: i64,

    feelslike_c: f64,

    feelslike_f: f64,

    windchill_c: f64,

    windchill_f: f64,

    heatindex_c: f64,

    heatindex_f: f64,

    dewpoint_c: f64,

    dewpoint_f: f64,

    vis_km: f64,

    vis_miles: f64,

    uv: f64,

    gust_mph: f64,

    gust_kph: f64,

    air_quality: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Condition {
    text: String,

    icon: String,

    code: i64,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Location {
    name: String,

    region: String,

    country: String,

    lat: f64,

    lon: f64,

    tz_id: String,

    localtime_epoch: i64,

    localtime: String,
}
