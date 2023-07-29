use super::font::FONT;

static DISPLAY_WIDTH: usize = 84;
static DISPLAY_HEIGHT: usize = 48;

pub struct DisplayBuffer {
    pub data: [u8; 504],
}

impl DisplayBuffer {
    pub fn new() -> Self {
        DisplayBuffer { data: [0; 504] }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize) {
        if x >= DISPLAY_WIDTH || y >=DISPLAY_HEIGHT {
            return;
        }

        let pos = (y / 8, y % 8);
        let cur = &mut self.data[pos.0 * DISPLAY_WIDTH + x];
        *cur = *cur | 1u8 << pos.1;
    }

    pub fn clear_pixel(&mut self, x: usize, y: usize) {
        if x >= DISPLAY_WIDTH || y >=DISPLAY_HEIGHT {
            return;
        }

        let pos = (y / 8, y % 8);
        let cur = &mut self.data[pos.0 * DISPLAY_WIDTH + x];
        *cur = *cur & (!(1u8 << pos.1));
    }

    pub fn text_mode_put_text(&mut self, text: &str, x: usize, y: usize) {
        let str_iter = text
            .as_bytes()
            .iter()
            .map(| c | if c.is_ascii() { c } else { &0x00 } )
            .enumerate();

        for (pos, ch) in str_iter {
            let buf_start = DISPLAY_WIDTH * y + 6 * x + pos * 6;
            let chunk: &mut [u8; 5] = &mut self.data[
                buf_start..buf_start + 5
            ].try_into().unwrap();
            *chunk = FONT[*ch as usize];
        }
    }
}