
use std::env;
use dotenv::dotenv;
use reqwest::{self, Error};

// For Deserializing the data
use serde_json::{Result, Value};
use druid::widget::{Label, Button};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};



fn main(){



    loop{
    let city="Stockholm";
    println!("Current City : {}",city);
   
        let current_weather=dressme::fetch_current_weather(city,"");
        println!("Current Weather : {}",current_weather);
        let recommended_dress = dressme::dress_maker(current_weather);
        println!("Recommended Layers : {}", recommended_dress);
        println!("-----------------------------------------");

        std::thread::sleep(std::time::Duration::from_secs(600));
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;




    #[test]
    fn test_negative_weather() {
        let temp = -10.0;
        let out = dressme::dress_maker(temp);
        assert_eq!(out, "4+ Layers!");      
    }   

    #[test]
    fn test_slightly_positive_weather() {
        let temp = 2.5;
        let out = dressme::dress_maker(temp);
        assert_eq!(out, "3 Layers!");      
    }   



    #[test]
    fn test_dress_maker_mild_summer() {
        let temp = 20.0;
        let out = dressme::dress_maker(temp);
        assert_eq!(out, "1 Layer");      
    }   

    #[test]
    fn test_dress_maker_incorrect() {
        let temp = 21.0;
        let out = dressme::dress_maker(temp);
        assert_ne!(out, "Shorts!");      
    }   

    #[test]
    fn test_current_weather() {
        dotenv().ok();
        let city="Malaga";
        let current_weather=dressme::fetch_current_weather(city,"");
        assert!(current_weather>0.0);      
    
    }
}


/*

Todo :

Integrate SL API to get traffic data

*/