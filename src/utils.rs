use crate::data::CoinData ;
//use tabular::row ;
//use prettytable to print out table 
use prettytable::{Cell, Row, Table};
use url::{Url, ParseError};



// Input : an string slice of an url
// Return : an Url or just an error 
pub fn parse_url(url: &str) -> Result<Url , ParseError> {
    match Url::parse(url){
        Ok(url) => Ok(url),
        //Handling Relative URL Error 
        Err(e) if e == ParseError::RelativeUrlWithoutBase 
        => { 
            let url_with_base = format!("{}{}", "http://", url);
            Url::parse(url_with_base.as_str())
        }
        Err(e) => Err(e),
    }
}



pub fn pretty_print(data: Vec<CoinData>) -> Result<() , ()> {
    
    //create table for print data 
    let mut table = Table::new();
    // for item in data{
    //     table.add_row(row![
    //         item.market_data.current_price.get("usd").unwrap().to_string(),
    //         format!("24h Price Change: {:.4}%", item.market_data.price_change_24h.get("usd").unwrap())
    //     ]);
    // }
    for item in data {
        let usd_price = item.market_data.current_price.get("usd").unwrap().to_string();
        let price_change = format!("24h Price Change: {:.4}%", item.market_data.price_change_24h.get("usd").unwrap());
    
        let row = Row::new(vec![
            Cell::new(&usd_price),
            Cell::new(&price_change),
        ]);
    }
   

    table.printstd();
    return Ok(());
}