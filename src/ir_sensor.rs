use arduino_hal::{port::{Pin, mode::Analog}, hal::port::PC0, Adc};

//generalizar o pino
pub struct IRSensor 
{
    analog_pin: Pin<Analog, PC0>
}

impl IRSensor
{
    pub fn new(analog_pin: Pin<Analog, PC0>) -> IRSensor {
        IRSensor { analog_pin }
    }

    pub fn raw_read(&self, adc_ref: &mut Adc ) -> u16 {
        self.analog_pin.analog_read(adc_ref)
    }
}