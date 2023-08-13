use crate::data::CoinData ;
//use tabular::row ;
//use prettytable to print out table 
use prettytable::{Cell, Row, Table};


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