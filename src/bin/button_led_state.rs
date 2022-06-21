use std::error::Error;
use std::time::Duration;

use rppal::gpio::{Gpio, Trigger};

const GPIO_LED: u8 = 17;
const GPIO_BUTTON: u8 = 18;

fn main() -> Result<(), Box<dyn Error>> {
    let mut led_pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let mut button_pin = Gpio::new()?.get(GPIO_BUTTON)?.into_input_pullup();

    led_pin.set_high();
    button_pin.set_interrupt(Trigger::FallingEdge);

    let mut state = false;

    loop {
        if let Ok(Some(_)) = button_pin.poll_interrupt(true, Some(Duration::from_millis(500))) {
            state = !state;

            if state {
                led_pin.set_low();
            } else {
                led_pin.set_high();
            }
        }

    }
}
