#![no_std]
#![no_main]

mod car;

use panic_halt as _;

use car::{Car, motor::Motor};

use arduino_hal::{
    delay_ms,
    simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer1Pwm, Timer2Pwm},
};

// fn motor_profile(motor: &mut impl ControllableMotor, count: u32) {
//     let rem = count.rem(768);
//     match rem {
//         0..=255 => motor.set_state(MotorState::Forward(rem as u8)),
//         256..=511 => motor.set_state(MotorState::Backward((rem - 256) as u8)),
//         _ => motor.set_state(MotorState::Stopped),
//     }
//     arduino_hal::delay_ms(20);
// }

#[arduino_hal::entry]
fn main() -> ! {
    //colocar como globais?
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let mut car = Car::default();

    let mut count = 0;
    loop {
        // motor_profile(&mut motor, count);
        // count += 1;
        // motor.set_state(MotorState::Forward(128));
        // delay_ms(500);
        // motor.set_state(MotorState::Forward(255));
        // delay_ms(500);
        // motor.set_state(MotorState::Backward(200));
        // delay_ms(500);
        // motor.set_state(MotorState::Stopped);
        // delay_ms(500);
    }
}
