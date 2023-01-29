//circuito do l293d tem Vcc1 trocado com Vcc2!!!!!
use arduino_hal::{
    port::{mode::PwmOutput, Pin},
    simple_pwm::PwmPinOps,
};

//controle só digital, depois colocar pwm
pub struct Motor<PlusPort, PlusTimer, MinusPort, MinusTimer> 
where
    PlusPort: PwmPinOps<PlusTimer>,
    MinusPort: PwmPinOps<MinusTimer>
{
    plus_pin: Pin<PwmOutput<PlusTimer>, PlusPort>,
    minus_pin: Pin<PwmOutput<MinusTimer>, MinusPort>,
}

enum MotorState {
    Forward(u8),
    Backward(u8),
    Stopped,
}

impl<PlusPort, PlusTimer, MinusPort, MinusTimer> Motor<PlusPort, PlusTimer, MinusPort, MinusTimer>
where
    MinusPort: PwmPinOps<MinusTimer>,
    PlusPort: PwmPinOps<PlusTimer>,
{
    pub fn new(
        plus_pin: Pin<PwmOutput<PlusTimer>, PlusPort>,
        minus_pin: Pin<PwmOutput<MinusTimer>, MinusPort>,
    ) -> Motor<PlusPort, PlusTimer, MinusPort, MinusTimer> {
        Motor {
            plus_pin,
            minus_pin,
        }
    }

    fn set_state(&mut self, state: MotorState) {
        match state {
            MotorState::Forward(v) => {
                self.plus_pin.set_duty(v);
                self.minus_pin.set_duty(0);
            }
            MotorState::Backward(v) => {
                self.plus_pin.set_duty(0);
                self.minus_pin.set_duty(v);
            }
            MotorState::Stopped => {
                self.plus_pin.set_duty(0);
                self.minus_pin.set_duty(0);
            }
        }
        self.plus_pin.enable();
        self.minus_pin.enable();
    }
}

mod normalized_power {
    use derive_more::Neg;
    #[derive(Clone, Copy, Neg)]
    pub struct NormalizedPower {
        power: f32,
    }

    impl From<f32> for NormalizedPower {
        fn from(power: f32) -> Self {
            NormalizedPower {
                power: power.clamp(-1_f32, 1_f32),
            }
        }
    }
    use micromath::F32Ext;
    impl From<NormalizedPower> for super::MotorState {
        fn from(power: NormalizedPower) -> Self {
            let is_positive = power.power.is_sign_positive();
            let int_power = (power.power.abs() * 255.0).round() as u8;
            match (is_positive, int_power) {
                (true, 1..=255) => Self::Forward(int_power),
                (false, 1..=255) => Self::Backward(int_power),
                (_, 0) => Self::Stopped,
            }
        }
    }
}

pub use normalized_power::NormalizedPower;

pub trait ControllableMotor {
    fn set_power(&mut self, power: NormalizedPower);
}

impl<PlusPort, PlusTimer, MinusPort, MinusTimer> ControllableMotor
    for Motor<PlusPort, PlusTimer, MinusPort, MinusTimer>
where
    PlusPort: PwmPinOps<PlusTimer>,
    MinusPort: PwmPinOps<MinusTimer>,
{
    fn set_power(&mut self, power: NormalizedPower) {
        self.set_state(power.into());
    }
}
