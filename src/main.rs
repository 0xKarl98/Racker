use clap_conf::clap_app;
use Racker::data::update_data;
use Racker:: utils::parse_url;

use failure::Fallible;
use futures::executor::block_on;
use std::{thread, time::Duration};


fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            //process::exit(1);
        }
    }
}


fn run() -> Fallible<()> {
    let args = clap_app!(cryptowatcher =>
        (author: "Karl Yu")
        (about: "A crypto tracker written in Rust.")
        (@arg tokens: +takes_value "you can add aditional tokens, f.e ripple,stellar")
        )
        .get_matches_safe();
    
        //default tokens are initialized into a vector 
        let mut tokens = [
            "bitcoin".to_owned(),
            "ethereum".to_owned(),
            "litecoin".to_owned(),
        ]
        .to_vec();

        //split the new tokens with comma and 
        //push them to tokens list 
        let base_url = "https://api.coingecko.com/api/v3/coins/";
        if let Some(tkns) = args.unwrap().value_of("tokens") {
            for split in tkns.split(",") {
                if split != "" {
                    tokens.push(split.to_owned());
                }
            }
        }
    
        let mut urls = Vec::new();
        for item in tokens {
            urls.push(parse_url(&format!("{}{}", base_url, item)).unwrap());
        }
 
    
        println!("Welcome to cryptowatcher 👋\n
                please wait a little to initial data be fetched\n
                thanks for your patience"
        );
    
        let update = Box::new(move || {
            block_on(update_data(&urls)).unwrap();
            thread::sleep(Duration::from_millis(1000));
        });
    
        let scheduler = thread::spawn(move || loop {
            let thread = thread::spawn(update.to_owned());
            thread.join().expect("Thread panicked");
        });
    
        scheduler.join().expect("Scheduler panicked");
    
        return Ok(());
}


