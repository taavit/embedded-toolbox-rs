use embedded_hal::{digital::v2::OutputPin, blocking::spi::Write};

pub mod graphics;
pub mod font;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Instruction {
    Nop = 0x00,
    FunctionSet = 0x20,
    DisplayControl = 0x08,
    Contrast = 0x80,
    BiasMode = 0x10,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum ExtendedSet {
    EXTENDED = 0x01,
    STANDARD = 0x00,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Addressing {
    HORIZONTAL = 0x02,
    VERTICAL   = 0x00,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DisplayMode {
    Blank   = 0x00,
    Normal  = 0x04,
    AllOn   = 0x01,
    Inverse = 0x05,
}

pub struct Pcd8544<SPI, CE, DC, RST> {
    spi: SPI,
    lcd_rst: RST,
    lcd_ce: CE,
    lcd_dc: DC,
}

impl<SPI, CE, DC, RST> Pcd8544<SPI, CE, DC, RST>
where
    SPI: Write<u8>,
    CE: OutputPin,
    DC: OutputPin,
    RST: OutputPin,
{
    pub fn new(
        spi: SPI,
        lcd_ce: CE,
        lcd_dc: DC,
        lcd_rst: RST,
    ) -> Result<Self, ()> {
        let mut n = Self {
            spi,
            lcd_ce,
            lcd_dc,
            lcd_rst,
        };
        n.reset()?;

        Ok(n)
    }

    pub fn reset(&mut self) -> Result<(), ()> {
        self.lcd_rst.set_low().map_err(|_| ())?;
        self.lcd_rst.set_high().map_err(|_| ())?;

        Ok(())
    }

    pub fn init(&mut self) -> Result<(), ()> {
        self.reset()?;

        self.lcd_cmd(Instruction::FunctionSet as u8 | ExtendedSet::EXTENDED as u8)?;
        self.set_bias(0x04)?;
        self.set_contrast(0x3f)?;
        self.lcd_cmd(Instruction::FunctionSet as u8 | ExtendedSet::STANDARD as u8)?;
        self.lcd_cmd(Instruction::DisplayControl as u8 | DisplayMode::Normal as u8)?;

        Ok(())
    }

    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), ()> {
        self.lcd_cmd(Instruction::Contrast as u8 | contrast).map_err(|_| ())?;

        Ok(())
    }

    fn set_bias(&mut self, bias: u8) -> Result<(), ()> {
        self.lcd_cmd(Instruction::BiasMode as u8| bias).map_err(|_| ())?;

        Ok(())
    }

    fn lcd_cmd(&mut self, cmd: u8) -> Result<(), ()>
    {
        self.lcd_ce.set_low().map_err(|_| ())?;
        self.lcd_dc.set_low().map_err(|_| ())?;
        self.spi.write(&mut [cmd]).map_err(|_| ())?;
        self.lcd_ce.set_high().map_err(|_| ())?;
        self.lcd_dc.set_high().map_err(|_| ())?;

        Ok(())
    }

    pub fn lcd_data(&mut self, data: &mut [u8; 504]) -> Result<(), ()>
    {
        self.lcd_dc.set_high().map_err(|_| ())?;
        self.lcd_ce.set_low().map_err(|_| ())?;
        self.spi.write(data).map_err(|_| ())?;
        self.lcd_ce.set_high().map_err(|_| ())?;

        Ok(())
    }
}
