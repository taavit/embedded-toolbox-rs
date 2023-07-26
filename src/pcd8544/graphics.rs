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
        for (pos, ch) in text.as_bytes().iter().map(| c | if c.is_ascii() { c } else { &0x00 } ).enumerate() {
            self.data[84 * y + 6*x + pos * 6 + 0] = FONT[*ch as usize][0];
            self.data[84 * y + 6*x + pos * 6 + 1] = FONT[*ch as usize][1];
            self.data[84 * y + 6*x + pos * 6 + 2] = FONT[*ch as usize][2];
            self.data[84 * y + 6*x + pos * 6 + 3] = FONT[*ch as usize][3];
            self.data[84 * y + 6*x + pos * 6 + 4] = FONT[*ch as usize][4];
        }
    }
}