
use reqwest;

// For Deserializing the data
use serde_json::{Result, Value};

fn fetchData(url: String) -> Result<()> {
     let resp: String = match reqwest::blocking::get(url) {
         Ok(resp) => resp.text().unwrap(),
         Err(err) => panic!("Error: {}", err)
     };
    let v: Value = serde_json::from_str(&resp)?;

    let current_weather: f64 = v["current"]["temp_c"].as_f64().unwrap();
    println!("Weather currently is at {} C", current_weather );
    dressMaker(current_weather);
    Ok(())

}


fn dressMaker(temp: f64){

    let out:&str = match temp {
        temp if temp>25.0 => "Shorts!",
        temp if temp>15.0 => "Light Jacket!",
        temp if temp>10.0 => "Autumn Jacket!",
        temp if temp>0.0 => "Winter Jacket!",
        _ => "God help you!"
    };
    println!("{}",out);
}


fn main(){
    let city="Malaga";
    let url: String = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", "ca6db37f82fc4cba9cf51956241909",city);
    println!("Current City : {}",city);
    fetchData(url);
}


#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // #[should_panic]
    // fn test_invalid_url() -> Result<()> {
    //     let city: &str="Kailasas";
    //     let url: String = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", "ca6db37f82fc4cba9cf51956241909",city);
    //     panic!("Oh no");
    // }


    #[test]
    fn test_valid_url() -> Result<()> {
        let city: &str="Dubrovnik";
        let url: String = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", "ca6db37f82fc4cba9cf51956241909",city);
        fetchData(url)
    }
}


/*

Todo :

Sends an email at 6am in the morning.
*/