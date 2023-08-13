use url::Url;
use serde::Deserialize;
use failure::Fallible;
use reqwest::blocking::Client;

//Deserialize trait tells rust to use serde to parse the json data
//Add support more data types later 
//we note those vars as pub , as they are used in util.rs
#[derive(Deserialize, Debug)]
pub struct MarketData {
    pub current_price: f64,
    all_time_high: f64,
    market_cap: f64 , 
    market_cap_rank: u32,
    high_24h :f64 ,
    low_24h : f64,
    pub price_change_24h :f64, 
    
}

#[derive(Deserialize, Debug)]
pub struct CoinData {
    id : String ,
    symbol : String,
    name : String,
    pub market_data : MarketData ,
}


//get crypto currency data from a given url 
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

// pub  async fn update_data()  ->  Fallible<()> {
    

// }







