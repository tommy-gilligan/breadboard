#![no_std]
use ili9488::Ili9488;
use xpt2046::Xpt2046;
use touchscreen::{TouchEvent, TouchEventType, Touchscreen};
use embedded_graphics_core::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, Size},
    Pixel,
};
use embedded_hal::digital::OutputPin;

pub struct RedScreen<
    LS: embedded_hal::spi::SpiDevice,
    TS: embedded_hal::spi::SpiDevice,
    DC: OutputPin,
> {
    ili_9488: Ili9488<LS, DC>,
    xpt_2046: Xpt2046<TS>,
    last_touch: Option<(i32, i32)>,
}

impl<LS: embedded_hal::spi::SpiDevice, TS: embedded_hal::spi::SpiDevice, DC: OutputPin>
    RedScreen<LS, TS, DC>
{
    pub fn new<RST: OutputPin, D: embedded_hal::delay::DelayUs>(
        lcd_spi_device: LS,
        lcd_dc: DC,
        lcd_rst: RST,
        touch_spi_device: TS,
        delay: D,
    ) -> Self {
        Self {
            ili_9488: Ili9488::new(lcd_spi_device, lcd_dc, lcd_rst, delay),
            xpt_2046: Xpt2046::new(touch_spi_device),
            last_touch: None,
        }
    }
}

impl<LS: embedded_hal::spi::SpiDevice, TS: embedded_hal::spi::SpiDevice, DC: OutputPin> DrawTarget
    for RedScreen<LS, TS, DC>
{
    type Color = Rgb888;
    type Error = core::convert::Infallible;
    fn draw_iter<I: IntoIterator<Item = Pixel<<Self as DrawTarget>::Color>>>(
        &mut self,
        i: I,
    ) -> Result<(), <Self as DrawTarget>::Error> {
        self.ili_9488.draw_iter(i)
    }
}

impl<LS: embedded_hal::spi::SpiDevice, TS: embedded_hal::spi::SpiDevice, DC: OutputPin>
    OriginDimensions for RedScreen<LS, TS, DC>
{
    fn size(&self) -> Size {
        Size::new(480, 320)
    }
}

impl<LS: embedded_hal::spi::SpiDevice, TS: embedded_hal::spi::SpiDevice, DC: OutputPin> Touchscreen
    for RedScreen<LS, TS, DC>
{
    fn get_touch_event(&mut self) -> Option<TouchEvent> {
        match convert(self.xpt_2046.get().unwrap()) {
            Some((x, y)) => {
                let result = Some(TouchEvent {
                    x,
                    y,
                    r#type: if self.last_touch.is_some() {
                        TouchEventType::Move
                    } else {
                        TouchEventType::Start
                    },
                });
                self.last_touch = Some((x, y));

                result
            }
            None => {
                if let Some((last_x, last_y)) = self.last_touch {
                    self.last_touch = None;

                    Some(TouchEvent {
                        x: last_x,
                        y: last_y,
                        r#type: TouchEventType::End,
                    })
                } else {
                    None
                }
            }
        }
    }
}

fn convert((x, y): (u16, u16)) -> Option<(i32, i32)> {
    if x < 250 || y < 230 || x > 4000 || y > 3900 {
        return None;
    }

    Some((
        ((x - 250).wrapping_shr(6) * 9).into(),
        ((y - 230).wrapping_shr(6) * 6).into(),
    ))
}

#[cfg(test)]
mod test {
    extern crate std;

    #[test]
    fn test_convert() {
        assert_eq!(super::convert((250, 230)), Some((0, 0)));
        assert_eq!(super::convert((3920, 3850)), Some((476, 322)));
    }

    #[test]
    fn test_convert_out_of_range() {
        assert_eq!(super::convert((200, 200)), None);
        assert_eq!(super::convert((4000, 4000)), None);
    }
}
