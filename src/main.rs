
use std::env;
use dotenv::dotenv;
use reqwest::{self, Error};

// For Deserializing the data
use serde_json::{Result, Value};
use druid::widget::{Label, Button};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};



fn fetchCurrentWeather(url: String) -> f64 {
     
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
    // dressMaker(current_weather);
    current_weather

}


fn build_ui() -> impl Widget<String> {
    // Create a label with an initial text
    let label = Label::new(|data: &String, _env: &druid::Env| data.to_string());

    // Create a button
    let button = Button::new("Click me").on_click(|_ctx, data: &mut String, _env| {
        data.push_str(" Rustc!");//adds " Rust!" to the end of the text
    });
    // Create a button to reset the text
    let reset_text_button = Button::new("Reset Text").on_click(|_ctx, data: &mut String, _env| {
        *data = "Hello, Again!".to_string();
    });

    // Create a vertical layout
    let layout = druid::widget::Flex::column()
        .with_child(label)
        .with_spacer(20.0)
        .with_child(button)
        .with_child(reset_text_button);

    layout
}



fn dressMaker(temp: f64) -> String {

    let out:&str = match temp {
        temp if temp>25.0 => "Shorts!",
        temp if temp>15.0 => "Light Jacket!",
        temp if temp>10.0 => "Autumn Jacket!",
        temp if temp>0.0 => "Winter Jacket!",
        _ => "God help you!"
    };
  String::from(out)
}


fn main(){
    dotenv().ok();
    let API_KEY:String=env::var("API_KEY").expect("API_KEY is not set.");
    let city="Stockholm";
    let url: String = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", API_KEY,city);
    println!("Current City : {}",city);
    let current_weather=fetchCurrentWeather(url);
    println!("Current Weather : {}",current_weather);
    let recommended_dress = dressMaker(current_weather);

    let main_window = WindowDesc::new(build_ui)
        .title(LocalizedString::new("Rust GUI App")) //App Title
        .window_size((300.0, 300.0)); //Window Size

    let DisplayText = format!(" Current Weather in {0} : {1},\n it is recommended to wear {2}\n 'n", city, current_weather, recommended_dress);
    // Launch the application
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(DisplayText.to_string());//Text it will display





}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dressmaker() {
        let temp = 20.0;
        let out = dressMaker(temp);
        assert_eq!(out, "Light Jacket!");      
    }   

    #[test]
    fn test_dressmaker_incorrect() {
        let temp = 21.0;
        let out = dressMaker(temp);
        assert_ne!(out, "Shorts!");      
    }   

    #[test]
    fn test_current_weather() {
        dotenv().ok();
        let city="Stockholm";
        let API_KEY:String=env::var("API_KEY").expect("API_KEY is not set.");
        let url: String = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", API_KEY,city);
        let current_weather=fetchCurrentWeather(url);
        assert!(current_weather>0.0);      
    }
}


/*

Todo :

Integrate SL API to get traffic data

*/