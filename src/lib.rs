
use std::env;
use dotenv::dotenv;
use reqwest::{self, Error};

// For Deserializing the data
use serde_json::{Result, Value};
use druid::widget::{Label, Button};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};



pub fn fetch_current_weather(city: &str, api_key: &str) -> f64 {         
   
    dotenv().ok();
    // Get the API key 
     let api_key = match api_key {
         "" => env::var("API_KEY").expect("api_key is not set."),
         k => k.to_string(),
     };

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
    //println!("Weather currently is at {} C", current_weather );
    // dress_maker(current_weather);
    current_weather

}


pub fn dress_maker(temp: f64) -> String {

    let out:&str = match temp {
        temp if temp>25.0 => "0 Layer",
        temp if temp>15.0 => "1 Layer",
        temp if temp>10.0 => "2 Layer",
        temp if temp>0.0 => "3 Layers!",
        _ => "4+ Layers!"
    };
  String::from(out)




  
}





/*

Todo :

Integrate SL API to get traffic data

*/