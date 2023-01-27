//circuito do l293d tem Vcc1 trocado com Vcc2!!!!!
use arduino_hal::{
    port::{mode::PwmOutput, Pin},
    simple_pwm::PwmPinOps,
};

//controle só digital, depois colocar pwm
pub struct Motor<PlusPort, PlusTimer, MinusPort, MinusTimer> {
    plus_pin: Pin<PwmOutput<PlusTimer>, PlusPort>,
    minus_pin: Pin<PwmOutput<MinusTimer>, MinusPort>,
}

pub enum MotorState {
    Forward(u8),
    Backward(u8),
    Still,
}

pub trait SettableMotor {
    fn set_state(&mut self, state: MotorState);
}

impl<PlusPort, PlusTimer, MinusPort, MinusTimer> Motor<PlusPort, PlusTimer, MinusPort, MinusTimer> {
    pub fn new(
        plus_pin: Pin<PwmOutput<PlusTimer>, PlusPort>,
        minus_pin: Pin<PwmOutput<MinusTimer>, MinusPort>,
    ) -> Motor<PlusPort, PlusTimer, MinusPort, MinusTimer> {
        Motor {
            plus_pin,
            minus_pin,
        }
    }
}

impl<PlusPort, PlusTimer, MinusPort, MinusTimer> SettableMotor
    for Motor<PlusPort, PlusTimer, MinusPort, MinusTimer>
where
    PlusPort: PwmPinOps<PlusTimer>,
    MinusPort: PwmPinOps<MinusTimer>,
{
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
            MotorState::Still => {
                self.plus_pin.set_duty(0);
                self.minus_pin.set_duty(0);
            }
        }
        self.plus_pin.enable();
        self.minus_pin.enable();
    }
}
