#![no_std]
#![no_main]


use panic_halt as _;

mod car;
use car::{Car, ControllableCar};

mod dist_sensor;
use dist_sensor::DistanceSensor;
use arduino_hal::{delay_ms, simple_pwm::IntoPwmPin};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let sensor = dist_sensor::ir_sensor::IRSensor::new(pins.a0.into_analog_input(&mut adc));

    loop {
        let val = sensor.calibrated_read(&mut adc);
        let val_write = (val.val()*10.0_f32) as u32;
        ufmt::uwriteln!(&mut serial, "Medida: {} mm", val_write).unwrap();
        delay_ms(500); 
    }
}
