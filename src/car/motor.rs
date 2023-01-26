use arduino_hal::port::{Pin, mode::{Output, Input, Floating, Io}, PinOps};

//circuito do l293d tem Vcc1 trocado com Vcc2!!!!!
struct Motor<Port1, Port2> {
    plus_pin: Pin<Output, Port1>,
    minus_pin: Pin<Output, Port2>
}

impl<Port1: PinOps, Port2: PinOps> Motor<Port1, Port2>  {
    fn new<PM: Io, MM: Io>(plus_pin: Pin<PM, Port1>, minus_pin: Pin<MM, Port2>) -> Motor<Port1, Port2> {
        Motor { plus_pin: plus_pin.into_output(), minus_pin: minus_pin.into_output() }
    }


}