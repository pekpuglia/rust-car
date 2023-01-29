pub mod motor;
use motor::{ControllableMotor, NormalizedPower};

pub struct Car<Lmot, Rmot> {
    left_motor: Lmot,
    right_motor: Rmot,
}

pub enum CommonMovements {
    Stopped,
    Straight(NormalizedPower),
    TurningOnADime { rpow: NormalizedPower },
}

impl<Lmot, Rmot> Car<Lmot, Rmot>
where
    Lmot: ControllableMotor,
    Rmot: ControllableMotor,
{
    pub fn new(lmot: Lmot, rmot: Rmot) -> Self {
        let mut ret = Car {
            left_motor: lmot,
            right_motor: rmot,
        };
        ret.left_motor.set_power(0_f32.into());
        ret.right_motor.set_power(0_f32.into());
        ret
    }
}

pub trait ControllableCar {
    fn set_power(&mut self, lpow: NormalizedPower, rpow: NormalizedPower);
    fn set_movement(&mut self, mov: CommonMovements);
}

impl<Lmot, Rmot> ControllableCar for Car<Lmot, Rmot>
where
    Lmot: ControllableMotor,
    Rmot: ControllableMotor,
{
    fn set_power(&mut self, lpow: NormalizedPower, rpow: NormalizedPower) {
        self.left_motor.set_power(lpow);
        self.right_motor.set_power(rpow);
    }

    fn set_movement(&mut self, mov: CommonMovements) {
        match mov {
            CommonMovements::Stopped => self.set_power(0_f32.into(), 0_f32.into()),
            CommonMovements::Straight(v) => self.set_power(v, v),
            CommonMovements::TurningOnADime { rpow } => self.set_power(-rpow, rpow),
        }
    }
}

use arduino_hal::{simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer1Pwm, Timer2Pwm}, port::{Pin, mode::PwmOutput}, hal::port::{PD6, PB1, PD5, PD3}};

type DefaultLeftMotor = motor::Motor<PD6, Timer0Pwm, PB1, Timer1Pwm>;

type DefaultRightMotor = motor::Motor<PD5, Timer0Pwm, PD3, Timer2Pwm>;

impl Default for Car<DefaultLeftMotor, DefaultRightMotor>
{
    fn default() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
        let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
        let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

        Self::new(
            motor::Motor::new(
                pins.d6.into_output().into_pwm(&timer0),
                pins.d9.into_output().into_pwm(&timer1),
            ),
            motor::Motor::new(
                pins.d5.into_output().into_pwm(&timer0),
                pins.d3.into_output().into_pwm(&timer2),
            ),
        )
    }
}
