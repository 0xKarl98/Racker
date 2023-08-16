use url::Url;
use serde::Deserialize;
use failure::Fallible;
use reqwest::blocking::Client;
use std::collections::HashMap;
use crate::utils::pretty_print;
//Deserialize trait tells rust to use serde to parse the json data
//Add support more data types later 
//we note those vars as pub , as they are used in util.rs
#[derive(Deserialize, Debug)]
pub struct MarketData {
    pub current_price: HashMap<String, f64>,
    all_time_high: HashMap<String, f64>,
    market_cap: HashMap<String, f64> , 
    market_cap_rank:usize ,
    high_24h :HashMap<String, f64> ,
    low_24h : HashMap<String, f64>,
    pub price_change_24h : HashMap<String, f64>, 
    
}

#[derive(Deserialize, Debug)]
pub struct CoinData {
    id : String ,
    symbol : String,
    name : String,
    pub market_data : MarketData ,
}


//get crypto currency data from a given url 
//Fallible will either return CoinData struct or an error 
pub async fn  get_data(url: &Url) -> Fallible<CoinData> {
    let coinData: CoinData = Client::new()
        .get(url.as_ref()) //gets the actual URL string
        .send() //sends http request 
        .unwrap()//get the actual response result 
        .json()//parse it as json 
        .unwrap() ;//get the parse result 

    //if it is good , return 
    return Ok(coinData);
}


//Update coinData in a batch manner 
pub  async fn update_data(urls: &Vec<Url>)  ->  Fallible<()> {
  
    let mut coin_Data : Vec<CoinData> = Vec::new() ;
    //put those coinData into this vector 
    for item in urls {
        let cData : CoinData =get_data(item).await?;
        coin_Data.push(cData);
    }
    //print those coin_Data 
    pretty_print(coin_Data).unwrap();
    return Ok(());

}







