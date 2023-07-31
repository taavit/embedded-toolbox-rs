use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use crate::sensors::humidity::HumidityReading;
use crate::sensors::temperature::TemperatureReading;

#[derive(Debug)]
pub enum Error {
    PinError,
    CrcError,
    Timeout,
}
pub struct Dht11<PIN, DELAY> {
    pin: PIN,
    delay: DELAY,
}

#[derive(Debug)]
pub struct Measurements {
    pub temperature: f64,
    pub humidity: f64,
}

impl HumidityReading for Measurements {
    fn get_humidity(&self) -> f64 {
        self.humidity
    }
}

impl TemperatureReading for Measurements {
    fn get_temperature(&self) -> f64 {
        self.temperature
    }
}

impl<PIN, DELAY> Dht11<PIN, DELAY>
where
    PIN: InputPin + OutputPin,
    DELAY: DelayMs<u16> + DelayUs<u16>,
{
    pub fn new(pin: PIN, delay: DELAY) -> Self {
        Self { pin, delay }
    }

    pub fn read_temperature_humidity(&mut self) -> Result<Measurements, Error> {
        self.send_measure_request()?;

        let mut data = [0u8; 5];

        for i in 0..40 {
            data[i / 8] <<= 1;
            if self.read_bit()? {
                data[i / 8] |= 1;
            }
        }

        let temperature = if data[2] & 0x80 != 0 {
            -i16::from(data[2] & 0x7f) * 10 + i16::from(data[3])
        } else {
            i16::from(data[2] & 0x7f) * 10 + i16::from(data[3])
        };

        let humidity = u16::from(data[0]) * 10 + u16::from(data[1]);

        let crc = data[0]
            .wrapping_add(data[1])
            .wrapping_add(data[2])
            .wrapping_add(data[3]);

        if crc != data[4] {
            return Err(Error::CrcError);
        }

        Ok(Measurements {
            temperature: f64::from(temperature) / 10.0,
            humidity: f64::from(humidity) / 10.0,
        })
    }

    fn send_measure_request(&mut self) -> Result<(), Error> {
        self.pin.set_high().map_err(|_| Error::PinError)?;
        self.delay.delay_ms(20);
        self.pin.set_low().map_err(|_| Error::PinError)?;
        self.delay.delay_ms(20);
        self.pin.set_high().map_err(|_| Error::PinError)?;
        self.delay.delay_us(40);

        self.read_bit()?;

        Ok(())
    }

    fn wait_for_raising(&mut self, us_timeout: u16) -> Result<u16, Error> {
        let mut elapsed: u16 = 0;

        while self.pin.is_low().map_err(|_| Error::PinError)? {
            if elapsed > us_timeout {
                return Err(Error::Timeout);
            }
            self.delay.delay_us(1);
            elapsed += 1;
        }
    
        Ok(elapsed)
    }
    
    fn wait_for_falling(&mut self, us_timeout: u16) -> Result<u16, Error> {
        let mut elapsed: u16 = 0;

        while self.pin.is_high().map_err(|_| Error::PinError)? {
            if elapsed > us_timeout {
                return Err(Error::Timeout);
            }
            self.delay.delay_us(1);
            elapsed += 1;
        }
    
        Ok(elapsed)
    }
    
    fn read_bit(&mut self) -> Result<bool, Error> {
        let low = self.wait_for_raising(1_000)?;
        let high = self.wait_for_falling(1_000)?;
    
        return Ok(high > low);
    }
}
