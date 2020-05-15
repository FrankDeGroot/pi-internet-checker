extern crate hyper;
extern crate gpio_cdev;
use gpio_cdev::*;
// use hyper::Client;
// use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn mirror_gpio(inputline: u32, outputline: u32) -> gpio_cdev::errors::Result<()> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let input = chip.get_line(inputline)?;
    let output = chip.get_line(outputline)?;
    let output_handle = output.request(LineRequestFlags::OUTPUT, 0, "mirror-gpio")?;
    output_handle.set_value(1)?;
    sleep(Duration::from_secs(1));
    output_handle.set_value(0)?;
    sleep(Duration::from_secs(1));
    //println!("for event");
    // for event in input.events(
    //     LineRequestFlags::INPUT,
    //     EventRequestFlags::BOTH_EDGES,
    //     "mirror-gpio",
    // )? {
    //     let evt = event?;
    //     println!("{:?}", evt);
    //     match evt.event_type() {
    //         EventType::RisingEdge => {
    //             output_handle.set_value(1)?;
    //         }
    //         EventType::FallingEdge => {
    //             output_handle.set_value(0)?;
    //         }
    //     }
    // }

    Ok(())
}

fn main() {
    loop {
        println!("start loop");
        match mirror_gpio(15, 18) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//     let blue_led = LED::new(18);
//     let green_led = LED::new(23);
//     let orange_led = LED::new(24);
//     let red_led = LED::new(25);
//     let client = Client::new();
//     loop {
//         blue_led.on();
//         green_led.off();
//         orange_led.off();
//         red_led.off();
//         let uri = "http://httpbin.org/ip".parse()?;
//         match client.get(uri).await {
//             Ok(resp) => {
//                 blue_led.off();
//                 println!("Response: {}", resp.status());
//                 if resp.status() == 200 {
//                     green_led.on();
//                     sleep(Duration::from_secs(1));
//                     green_led.off();
//                 } else {
//                     orange_led.on();
//                     sleep(Duration::from_secs(1));
//                     orange_led.off();
//                 }
//             }
//             Err(_) => {
//                 red_led.on();
//                 sleep(Duration::from_secs(1));
//                 red_led.off();
//             }
//         }
//         sleep(Duration::from_secs(3));
//     }
// }
