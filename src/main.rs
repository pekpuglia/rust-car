#![no_std]
#![no_main]

mod car;

use arduino_hal::simple_pwm::{IntoPwmPin, Timer0Pwm};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();
    let t = arduino_hal::simple_pwm::Timer2Pwm::new(dp.TC2, arduino_hal::simple_pwm::Prescaler::Prescale64);
    let mut pwm = pins.d11.into_output().into_pwm(timer);
    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
