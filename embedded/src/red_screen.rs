use crate::{ili9488::Ili9488, xpt2046::Xpt2046};
use application::touchscreen::{TouchEvent, TouchEventType, Touchscreen};
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
    last_touch: Option<(u16, u16)>,
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
        match crate::xpt2046::convert(self.xpt_2046.get().unwrap()) {
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
            },
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
