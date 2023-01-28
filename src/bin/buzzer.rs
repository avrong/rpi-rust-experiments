use std::thread;
use std::error::Error;
use std::time::Duration;

use rppal::gpio::Gpio;

const BEEP: u8 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    let mut beep_pin = Gpio::new()?.get(BEEP)?.into_output();

    loop {
        beep_pin.set_high();
        thread::sleep(Duration::from_millis(500));

        beep_pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}


