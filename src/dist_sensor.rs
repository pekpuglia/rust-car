pub struct RawReading {
    val: u16
}

impl From<u16> for RawReading {
    fn from(val: u16) -> Self {
        RawReading { val }
    }
}

pub struct Centimeter {
    val: f32
}

impl From<RawReading> for Centimeter {
    fn from(r: RawReading) -> Self {
        //nunca divide por 0 porque 0.01 / 2.32e-4 não é inteiro
        let val = 1.0_f32 / (-0.01_f32 + 2.3267e-4_f32 * (r.val as f32));
        Centimeter { val }
    }
}

impl Centimeter {
    pub fn val(self: &Centimeter) -> f32 {
        self.val
    }
}
use arduino_hal::Adc;
pub trait DistanceSensor {
    fn raw_read(&self, adc_ref: &mut Adc) -> RawReading;
    fn calibrated_read(&self, adc_ref: &mut Adc) -> Centimeter;
}

pub mod ir_sensor {
    use super::{DistanceSensor, RawReading, Centimeter};
    use arduino_hal::{port::{Pin, mode::Analog}, hal::port::PC0, Adc};
    
    //generalizar o pino
    //adicionar lógica para detectar quando não há obstáculo à vista
    pub struct IRSensor 
    {
        analog_pin: Pin<Analog, PC0>
    }
    impl IRSensor
    {
        pub fn new(analog_pin: Pin<Analog, PC0>) -> IRSensor {
            IRSensor { analog_pin }
        }
    }
    
    impl DistanceSensor for IRSensor {
        fn raw_read(&self, adc_ref: &mut Adc ) -> RawReading {
            self.analog_pin.analog_read(adc_ref).into()
        }
        
        fn calibrated_read(&self, adc_ref: &mut Adc) -> Centimeter {
            self.raw_read(adc_ref).into()
        }
        
    }
}

