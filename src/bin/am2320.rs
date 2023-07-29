use std::{thread::{self, sleep}, time::Duration};

use embedded_toolbox_rs::pcd8544::{Pcd8544, graphics::DisplayBuffer};
use rppal::{i2c::I2c, gpio::Gpio, spi::{Spi, Bus, SlaveSelect, Mode}};

fn main() {
    let mut spi = Spi::new(
        Bus::Spi1,
        SlaveSelect::Ss0,
        1_000_000,
        Mode::Mode0
    ).unwrap();
    
    let mut data = [0u8; 504];

    let mut rst_pin = Gpio::new().unwrap().get(13).unwrap().into_output();
    let mut ce_pin = Gpio::new().unwrap().get(16).unwrap().into_output();
    let mut dc_pin = Gpio::new().unwrap().get(26).unwrap().into_output();

    let mut bl = Gpio::new().unwrap().get(6).unwrap().into_output();
    bl.set_high();

    let mut nokia = Pcd8544::new(
        spi,
        ce_pin,
        dc_pin,
        rst_pin,
    ).unwrap();

    nokia.init().unwrap();
    nokia.lcd_data(&mut data);
    nokia.init();
    let mut display = DisplayBuffer { data };

    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(0x5c).unwrap();
    let mut buffer = [0u8; 8];
    loop {
        sleep(Duration::from_millis(250));
        i2c.write(&[0x03, 0x00, 0x04]);
        sleep(Duration::from_micros(1500));
        i2c.read(&mut buffer).unwrap();
        
        if buffer[0] == 0 {
            continue;
        }
        let h = i16::from_be_bytes(buffer[2..4].try_into().unwrap());
        let t = i16::from_be_bytes(buffer[4..6].try_into().unwrap());
        let h = f64::from(h) / 10.0;
        let t = f64::from(t) / 10.0;
        display.text_mode_put_text(format!("R. Hum. {:3.1}%", h).as_str(), 0, 0);
        display.text_mode_put_text(format!("Temp.   {:3.1}C", t).as_str(), 0, 1);
        nokia.lcd_data(&mut display.data);
        println!("{} {}", t, h);
    }
}
