use std::cell::RefCell;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_toolbox_rs::dht11::Dht11;
use rppal::gpio::{ Gpio, Mode, IoPin};
use rppal::hal::Delay;

struct IoPinWrapper {
    pin: RefCell<IoPin>,
}

impl IoPinWrapper {
    pub fn new(pin: IoPin) -> Self {
        Self { pin: RefCell::new(pin) }
    }
}

impl InputPin for IoPinWrapper {
    type Error = ();
    fn is_high(&self) -> Result<bool, Self::Error> {
        if self.pin.borrow().mode() == Mode::Output {
            self.pin.borrow_mut().set_mode(Mode::Input);
        }

        Ok(self.pin.borrow().is_high())
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        if self.pin.borrow().mode() == Mode::Output {
            self.pin.borrow_mut().set_mode(Mode::Input);
        }

        Ok(self.pin.borrow().is_low())
    }
}

impl OutputPin for IoPinWrapper {
    type Error = ();

    fn set_high(&mut self) -> Result<(), Self::Error> {
        if self.pin.borrow().mode() == Mode::Input {
            self.pin.borrow_mut().set_mode(Mode::Output);
        }

        self.pin.borrow_mut().set_high();

        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        if self.pin.borrow().mode() == Mode::Input {
            self.pin.borrow_mut().set_mode(Mode::Output);
        }

        self.pin.borrow_mut().set_low();

        Ok(())
    }
    fn set_state(&mut self, state: embedded_hal::digital::v2::PinState) -> Result<(), Self::Error> {
        if self.pin.borrow().mode() == Mode::Input {
            self.pin.borrow_mut().set_mode(Mode::Output);
        }

        self.pin.borrow_mut().set_state(state).map_err(|_| ())?;

        Ok(())
    }
}

fn main() {
    let pin = Gpio::new().unwrap().get(21).unwrap().into_io(Mode::Output);
    let delay = Delay::new();

    let wrapped_pin = IoPinWrapper::new(pin);

    let mut dht11 = Dht11::new(wrapped_pin, delay);

    dbg!(&dht11.read_temperature_humidity());
}
