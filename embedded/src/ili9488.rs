use core::convert::TryInto;
use embedded_graphics_core::{
    pixelcolor::{raw::RawU24, Rgb888},
    prelude::{DrawTarget, OriginDimensions, RawData, Size},
    Pixel,
};
use embedded_hal::digital::OutputPin;

#[derive(Debug)]
pub struct CommError;

pub struct Ili9488<S: embedded_hal::spi::SpiDevice, DC: OutputPin> {
    spi: S,
    dc: DC,
}

impl<S: embedded_hal::spi::SpiDevice, DC: OutputPin> Ili9488<S, DC> {
    fn data8(&mut self, d: u8) {
        self.spi
            .transaction(&mut [embedded_hal::spi::Operation::Write(&[d])])
            .unwrap();
    }

    fn data16(&mut self, d: u16) {
        self.spi
            .transaction(&mut [embedded_hal::spi::Operation::Write(&[(d >> 8) as u8])])
            .unwrap();
        self.spi
            .transaction(&mut [embedded_hal::spi::Operation::Write(&[(d & 0xff) as u8])])
            .unwrap();
    }

    fn data24(&mut self, r: u8, g: u8, b: u8) {
        self.spi
            .transaction(&mut [embedded_hal::spi::Operation::Write(&[r, g, b])])
            .unwrap();
    }

    fn cmd8(&mut self, d: u8) {
        self.dc.set_low().unwrap();
        self.data8(d);
        self.dc.set_high().unwrap();
    }

    pub fn new<RST: OutputPin, D: embedded_hal::delay::DelayUs>(
        spi_device: S,
        lcd_dc: DC,
        mut lcd_rst: RST,
        mut delay: D,
    ) -> Self {
        let mut result = Self {
            spi: spi_device,
            dc: lcd_dc,
        };

        lcd_rst.set_high().unwrap();
        delay.delay_ms(5);
        lcd_rst.set_low().unwrap();
        delay.delay_ms(15);
        lcd_rst.set_high().unwrap();
        delay.delay_ms(15);

        result.cmd8(0x1); //sw reset
        delay.delay_ms(120);
        result.cmd8(0x11); //Sleep out
        delay.delay_ms(120);

        result.cmd8(0x36); //Memory access control
        result.data8(56); //sets orientation/scan direction/flip
        result.cmd8(0x3A); //Pixel interface format
        result.data8(0x66);

        result.cmd8(0x29); //display on
                           //
        result.cmd8(0x2A); //set column start/end
        result.data8(0);
        result.data8(0); //start
        result.data8(1);
        result.data8(223); //end
        result.cmd8(0x2B); //set row start/end
        result.data8(0);
        result.data8(0); //start
        result.data8(1);
        result.data8(63); //end
        result.cmd8(0x2C); //draw!

        result
    }
}

impl<S: embedded_hal::spi::SpiDevice, DC: OutputPin> DrawTarget for Ili9488<S, DC> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            if let Ok((_x @ 0..=479, _y @ 0..=319)) = coord.try_into() {
                self.cmd8(0x2A); //set column start/end
                self.data16(coord.x as u16);
                self.data16(479_u16);
                self.cmd8(0x2B); //set row start/end
                self.data16(coord.y as u16);
                self.data16(319_u16);
                self.cmd8(0x2C); //draw!
                let mut a = core::iter::once(RawU24::from(color).into_inner());
                let rgb = a.next().unwrap();
                self.data24(
                    ((rgb & 0x00ff_0000) >> 16) as u8,
                    ((rgb & 0x0000_ff00) >> 8) as u8,
                    (rgb & 0x0000_00ff) as u8,
                );
            }
        }

        Ok(())
    }
}

impl<S: embedded_hal::spi::SpiDevice, DC: OutputPin> OriginDimensions for Ili9488<S, DC> {
    fn size(&self) -> Size {
        Size::new(480, 320)
    }
}
