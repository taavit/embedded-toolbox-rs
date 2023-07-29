use std::{thread, time::Duration};
use std::borrow::BorrowMut;
use embedded_toolbox_rs::pcd8544::Pcd8544;
use rppal::spi::{Spi, Bus, SlaveSelect, Mode};
use rppal::gpio::Gpio;

use rppal::i2c::I2c;

fn set_pixel(data: &mut [u8; 504], x: usize, y: usize, state: bool) {
    let pos = (y / 8, y % 8);
    let cur = data[pos.0 * 84 + x].borrow_mut();
    if state {
        *cur = *cur | 1u8 << pos.1;
    } else {
        *cur = *cur & (!(1u8 << pos.1));
    }
}

fn draw_square(
    data: &mut [u8; 504],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    fill: bool,
) {
    for cur_y in y..y + height {
        for cur_x in x..x + width {
            set_pixel(data, cur_x, cur_y, fill);
        }
    }
}

fn main() {
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(0x1d).unwrap();
    let mut buffer = [0u8; 1];
    let mut temp = [0u8; 2];
    let mut acc_x = [0u8; 2];
    let mut acc_y = [0u8; 2];
    let mut acc_z = [0u8; 2];
    // i2c.block_write(0x00, &[0x42, 0x43, 0x44, 0x45]).unwrap();
    // thread::sleep(Duration::from_millis(4000));
    i2c.block_read(0x0F, &mut buffer).unwrap();
    println!("Hello, world! {:?}", buffer);
    i2c.block_write(0x24, &[0x80|0x10]).unwrap();
    i2c.block_write(0x20, &[0x40|0x07]).unwrap();

    let mut spi = Spi::new(
        Bus::Spi1,
        SlaveSelect::Ss0,
        1_000_000,
        Mode::Mode0
    ).unwrap();
    
    let rst_pin = Gpio::new().unwrap().get(26).unwrap().into_output();
    let ce_pin = Gpio::new().unwrap().get(16).unwrap().into_output();
    let dc_pin = Gpio::new().unwrap().get(13).unwrap().into_output();

    let mut nokia = Pcd8544::new(
        spi,
        rst_pin,
        ce_pin,
        dc_pin,
    ).unwrap();

    let mut x: f64 = 42.0;
    let mut y: f64 = 24.0;

    let mut data = [0u8; 504];

    nokia.init().unwrap();

    loop {
        i2c.block_read(0x05 | 0x80, &mut temp).unwrap();
        println!("temperature! {:?}", i16::from_le_bytes(temp));

        i2c.block_read(0x28 | 0x80, &mut acc_x).unwrap();
        i2c.block_read(0x2a | 0x80, &mut acc_y).unwrap();
        i2c.block_read(0x2c | 0x80, &mut acc_z).unwrap();
        println!(
            "acc! {} {} {}",
            (f64::from(i16::from_le_bytes(acc_x)) * 2.0) / 32678.0,
            (f64::from(i16::from_le_bytes(acc_y)) * 2.0) / 32678.0,
            (f64::from(i16::from_le_bytes(acc_z)) * 2.0) / 32678.0,
        );

        draw_square(&mut data, x as usize, y as usize, 4, 4, false);
        let x_acc = (f64::from(i16::from_le_bytes(acc_x)) * 2.0) / 32678.0;
        let z_acc = (f64::from(i16::from_le_bytes(acc_z)) * 2.0) / 32678.0;

        x -= x_acc * 5.0;
        y += z_acc * 5.0;

        x = x.clamp(0.0, 83.0);
        y = y.clamp(0.0, 47.0);

        draw_square(&mut data, x as usize, y as usize, 4, 4, true);
        nokia.lcd_data(&mut data);

        thread::sleep(Duration::from_millis(100));
    }
}
