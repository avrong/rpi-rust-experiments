use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

const GPIO_RED: u8 = 22;
const GPIO_GREEN: u8 = 18;
const GPIO_BLUE: u8 = 27;

const PWM_FREQ: f64 = 100.0;

fn main() -> Result<(), Box<dyn Error>> {
    let mut red_pin = Gpio::new()?.get(GPIO_RED)?.into_output();
    let mut green_pin = Gpio::new()?.get(GPIO_GREEN)?.into_output();
    let mut blue_pin = Gpio::new()?.get(GPIO_BLUE)?.into_output();

    let (ri, gi, bi) = (255, 192, 203); // pink

    red_pin.set_pwm_frequency(PWM_FREQ, ri as f64 / 255.0)?;
    green_pin.set_pwm_frequency(PWM_FREQ, gi as f64 / 255.0)?;
    blue_pin.set_pwm_frequency(PWM_FREQ, bi as f64 / 255.0)?;

    thread::sleep(Duration::from_secs(5));

    Ok(())
}


