#![no_std]
#![no_main]


use panic_halt as _;

mod car;
use car::{Car, ControllableCar};

mod ir_sensor;

use arduino_hal::{delay_ms};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let a0 = pins.a0.into_analog_input(&mut adc);

    loop {
        let val = a0.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "Medida: {}", val).unwrap();
        delay_ms(500);
    }
}
