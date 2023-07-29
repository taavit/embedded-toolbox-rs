use embedded_toolbox_rs::pcd8544::Pcd8544;
use embedded_toolbox_rs::pcd8544::graphics::DisplayBuffer;
use rppal::spi::{Spi, Bus, SlaveSelect, Mode};
use rppal::gpio::{Gpio, OutputPin};
use std::borrow::BorrowMut;
use std::{thread, time::Duration};

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
    let mut spi = Spi::new(
        Bus::Spi1,
        SlaveSelect::Ss0,
        1_000_000,
        Mode::Mode0
    ).unwrap();
    
    let mut rst_pin = Gpio::new().unwrap().get(13).unwrap().into_output();
    let mut ce_pin = Gpio::new().unwrap().get(16).unwrap().into_output();
    let mut dc_pin = Gpio::new().unwrap().get(26).unwrap().into_output();

    let mut bl = Gpio::new().unwrap().get(6).unwrap().into_output().set_low();

    let mut nokia = Pcd8544::new(
        spi,
        ce_pin,
        dc_pin,
        rst_pin,
    ).unwrap();

    nokia.init().unwrap();
    let mut data = [0u8; 504];

    // let mut idx = 0u8;
    // for e in data.as_mut_slice() {
    //     *e = idx;
    //     idx = idx.wrapping_add(8);
    // }

    nokia.lcd_data(&mut data);
    let mut x: i8 = 0;
    let mut y: i8 = 0;
    let mut direction_x: i8 = 1;
    let mut direction_y: i8 = 1;
    let mut display = DisplayBuffer { data };
    loop {
            x = x.wrapping_add(1);
            y = x ^ y;
            display.text_mode_put_text(format!("Acc x: {:4}", x).as_str(), 0, 0);
            display.text_mode_put_text(format!("Acc y: {:4}", y).as_str(), 0, 1);
            display.text_mode_put_text(format!("Acc z: {:4}", x.overflowing_mul(y).0).as_str(), 0, 2);
            display.text_mode_put_text(format!("Acc a: {:4}", x.overflowing_sub(y).0).as_str(), 0, 3);
            nokia.lcd_data(&mut display.data).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
}