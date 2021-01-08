use futures::executor::block_on;
use std::thread;
use std::time::Duration;
use yahoo_finance_api::*;

extern crate chrono;
use chrono::prelude::*;

#[macro_use]
extern crate colour;

fn main() {
    let connector = YahooConnector::new();

    const TICKER: &str = "BTC-USD";

    let get_quote = |x| {
        block_on(connector.get_latest_quotes(x, ""))
            .unwrap()
            .last_quote()
            .unwrap()
    };

    let mut last_quote = get_quote(TICKER);
    let mut current_quote;
    let mut rolling_average = last_quote.close;
    let mut time = NaiveDateTime::from_timestamp(last_quote.timestamp as i64, 0).time();

    print!("\x1B[2J\x1B[1;1H"); //clear console and set cursor to top
    println!("Data provided by Yahoo Financial. Time is local to the market that the ticker originates.");
    println!(
        "Bitcoin Price: ${} at {}:{}",
        last_quote.close,
        time.hour(), time.minute() 
    );

    loop {
        current_quote = get_quote(TICKER);
        if current_quote.timestamp > last_quote.timestamp {
            time = NaiveDateTime::from_timestamp(current_quote.timestamp as i64, 0).time();
            if current_quote.close > rolling_average {
                green_ln!(
                    "Bitcoin Price: ${} at {}:{}",
                    current_quote.close,
                    time.hour(), time.minute() 
                );
            } else if current_quote.close < rolling_average {
                red_ln!(
                    "Bitcoin Price: ${} at {}:{}",
                    current_quote.close,
                    time.hour(), time.minute() 
                );
            } else {
                println!(
                    "Bitcoin Price: ${} at {}:{}",
                    current_quote.close,
                    time.hour(), time.minute() 
                );
            }
            rolling_average = rolling_average * 0.9 + (current_quote.close * 0.1);
        }
        last_quote = current_quote;
        thread::sleep(Duration::from_secs_f32(60.0));
    }
}
