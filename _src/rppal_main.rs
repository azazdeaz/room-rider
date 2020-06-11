use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_EEP: u8 = 26;
const GPIO_IN1: u8 = 5; //lb
const GPIO_IN2: u8 = 6;
const GPIO_IN3: u8 = 13;
const GPIO_IN4: u8 = 19;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin_eep = Gpio::new()?.get(GPIO_EEP)?.into_output();
    let mut pin_in1 = Gpio::new()?.get(GPIO_IN1)?.into_output();
    // let mut pin_in2 = Gpio::new()?.get(GPIO_IN2)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    // thread::sleep(Duration::from_millis(2000));
    println!("up");
    pin_eep.set_high();
    pin_in1.set_high();
    thread::sleep(Duration::from_millis(2000));
    pin_in1.set_low();
    pin_eep.set_low();

    Ok(())
}
