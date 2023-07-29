use embedded_hal::blocking::i2c::WriteRead;

static ADRESS: u8 = 0x5c;

pub struct AM2320<I2C> {
    i2c: I2C,
}

pub struct Readings {
    pub temperature: f64,
    pub humidity: f64,
}

impl<I2C> AM2320<I2C>
where I2C: WriteRead {
    pub fn new(i2c: I2C) -> Self {
        AM2320 {
            i2c,
        }
    }

    pub fn read_sensor(&mut self) -> Result<Readings, ()> {
        let mut buffer = [0u8; 8];

        self
            .i2c
            .write_read(
                ADRESS,
                &[0x03, 0x00, 0x04],
                &mut buffer
            )
            .map_err(|_| ())?;

        let h = i16::from_be_bytes(buffer[2..4].try_into().unwrap());
        let t = i16::from_be_bytes(buffer[4..6].try_into().unwrap());

        Ok(
            Readings {
                temperature: f64::from(t) / 10.0,
                humidity: f64::from(h) / 10.0,
            })
    }
}