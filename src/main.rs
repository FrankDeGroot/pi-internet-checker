extern crate hyper;
extern crate rust_gpiozero;
use hyper::Client;
use rust_gpiozero::*;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let blue_led = LED::new(18);
    let green_led = LED::new(23);
    let orange_led = LED::new(24);
    let red_led = LED::new(25);
    let client = Client::new();
    loop {
        blue_led.on();
        green_led.off();
        orange_led.off();
        red_led.off();
        let uri = "http://httpbin.org/ip".parse()?;
        match client.get(uri).await {
            Ok(resp) => {
                blue_led.off();
                println!("Response: {}", resp.status());
                if resp.status() == 200 {
                    green_led.on();
                    sleep(Duration::from_secs(1));
                    green_led.off();
                } else {
                    orange_led.on();
                    sleep(Duration::from_secs(1));
                    orange_led.off();
                }
            }
            Err(_) => {
                red_led.on();
                sleep(Duration::from_secs(1));
                red_led.off();
            }
        }
        sleep(Duration::from_secs(3));
    }
}
