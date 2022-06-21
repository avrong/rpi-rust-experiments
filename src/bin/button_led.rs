use std::error::Error;

use rppal::gpio::Gpio;

const GPIO_LED: u8 = 17;
const GPIO_BUTTON: u8 = 18;

fn main() -> Result<(), Box<dyn Error>> {
    let mut led_pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let button_pin = Gpio::new()?.get(GPIO_BUTTON)?.into_input_pullup();

    loop {
        if button_pin.is_low() {
            led_pin.set_low();
        } else {
            led_pin.set_high();
        }
    }
}
