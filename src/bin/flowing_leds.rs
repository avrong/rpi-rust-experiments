use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

const GPIO_LEDS: [u8; 8] = [17, 18, 27, 22, 23, 24, 25, 4];

fn main() -> Result<(), Box<dyn Error>> {
    let mut leds = GPIO_LEDS.iter()
        .filter_map(|x| Some(Gpio::new().ok()?.get(*x).ok()?.into_output_high()))
        .collect::<Vec<_>>();

    for _ in 0..10 {
        for led in &mut leds {
            led.set_low();
            thread::sleep(Duration::from_millis(100));
            led.set_high();
        }
    }

    Ok(())
}
