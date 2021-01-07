use futures::executor::block_on;
use std::thread;
use std::time::Duration;
use yahoo_finance_api::*;
fn main() {
    let connector = YahooConnector::new();
    let response = connector.get_latest_quotes("BTC-USD", "");
    let quote = block_on(response).unwrap().last_quote().unwrap();
    println!("Bitcoin Price: ${}", quote.close);
    loop {
        let response = connector.get_latest_quotes("BTC-USD", "");
        thread::sleep(Duration::from_secs_f32(10.0));
        let quote = block_on(response).unwrap().last_quote().unwrap();
        println!("Bitcoin Price: ${}", quote.close);
    }
}
