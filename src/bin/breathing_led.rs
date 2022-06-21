use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::pwm::{Pwm, Channel};

fn main() -> Result<(), Box<dyn Error>> {
    let mut led_pin = Pwm::new(Channel::Pwm0)?;
    led_pin.set_frequency(1000.0, 0.0)?;
    led_pin.enable()?;

    for _ in 0..10 {
        for i in 0..=100 {
            led_pin.set_duty_cycle(i as f64 / 100.0)?;
            thread::sleep(Duration::from_millis(20));
        }

        for i in (0..=100).rev() {
            led_pin.set_duty_cycle(i as f64 / 100.0)?;
            thread::sleep(Duration::from_millis(20));
        }
    }

    Ok(())
}
