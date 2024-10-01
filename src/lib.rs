
use std::env;
use dotenv::dotenv;
use reqwest::{self, Error};

// For Deserializing the data
use serde_json::{Result, Value};
use druid::widget::{Label, Button};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};



pub fn fetch_current_weather(city: &str) -> f64 {
     dotenv().ok();
     let api_key:String=env::var("API_KEY").expect("api_key is not set.");
     let url: String = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", api_key,city);    
     
     let resp: String = match reqwest::blocking::get(url) {
         Ok(resp) => resp.text().unwrap(),
         Err(err) => panic!("Error: {}", err)
     };


    // Parse the JSON data 
    let v: Value = match serde_json::from_str(&resp) {
        Ok(v) => v,
        Err(err) => panic!("Error: {}", err)
    };

    // Save the temperature
    let current_weather: f64 = v["current"]["temp_c"].as_f64().unwrap();
    println!("Weather currently is at {} C", current_weather );
    // dress_maker(current_weather);
    current_weather

}


pub fn dress_maker(temp: f64) -> String {

    let out:&str = match temp {
        temp if temp>25.0 => "Shorts!",
        temp if temp>15.0 => "Light Jacket!",
        temp if temp>10.0 => "Autumn Jacket!",
        temp if temp>0.0 => "Winter Jacket!",
        _ => "God help you!"
    };
  String::from(out)
}





// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_dress_maker() {
//         let temp = 20.0;
//         let out = dress_maker(temp);
//         assert_eq!(out, "Light Jacket!");      
//     }   

//     #[test]
//     fn test_dress_maker_incorrect() {
//         let temp = 21.0;
//         let out = dress_maker(temp);
//         assert_ne!(out, "Shorts!");      
//     }   

//     #[test]
//     fn test_current_weather() {
//         dotenv().ok();
//         let city="Malaga";
//         let current_weather=fetch_current_weather(city);
//         assert!(current_weather>0.0);      
//     }
// }


/*

Todo :

Integrate SL API to get traffic data

*/