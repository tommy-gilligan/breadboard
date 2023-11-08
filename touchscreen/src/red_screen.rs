use crate::{TouchEvent, TouchEventType, Touchscreen};
use embedded_graphics_core::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, Size},
    Pixel,
};
use embedded_hal::digital::OutputPin;
use xpt2046::Xpt2046;

pub struct RedScreen<
    TS: embedded_hal::spi::SpiDevice,
    S: DrawTarget + OriginDimensions,
    C: Fn((u16, u16)) -> Option<(i32, i32)>,
> {
    screen: S,
    xpt_2046: Xpt2046<TS>,
    last_touch: Option<(i32, i32)>,
    calibration: C,
}

impl<
        TS: embedded_hal::spi::SpiDevice,
        S: DrawTarget + OriginDimensions,
        C: Fn((u16, u16)) -> Option<(i32, i32)>,
    > RedScreen<TS, S, C>
{
    pub fn new(screen: S, touch_spi_device: TS, calibration: C) -> Self {
        Self {
            screen,
            xpt_2046: Xpt2046::new(touch_spi_device),
            last_touch: None,
            calibration,
        }
    }
}

impl<
        TS: embedded_hal::spi::SpiDevice,
        S: DrawTarget + OriginDimensions,
        C: Fn((u16, u16)) -> Option<(i32, i32)>,
    > DrawTarget for RedScreen<TS, S, C>
{
    type Color = S::Color;
    type Error = S::Error;

    fn draw_iter<I: IntoIterator<Item = Pixel<<Self as DrawTarget>::Color>>>(
        &mut self,
        i: I,
    ) -> Result<(), <Self as DrawTarget>::Error> {
        self.screen.draw_iter(i)
    }
}

impl<
        TS: embedded_hal::spi::SpiDevice,
        S: DrawTarget + OriginDimensions,
        C: Fn((u16, u16)) -> Option<(i32, i32)>,
    > OriginDimensions for RedScreen<TS, S, C>
{
    fn size(&self) -> Size {
        self.screen.size()
    }
}

impl<
        TS: embedded_hal::spi::SpiDevice,
        S: DrawTarget + OriginDimensions,
        C: Fn((u16, u16)) -> Option<(i32, i32)>,
    > Touchscreen for RedScreen<TS, S, C>
{
    fn get_touch_event(&mut self) -> Option<TouchEvent> {
        match (self.calibration)(self.xpt_2046.get().unwrap()) {
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
