use core::fmt::Debug;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::DrawTarget;

use crate::touchscreen::{TouchEvent, Touchscreen};
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics_core::{prelude::{Point, Size}, primitives::Rectangle};
use embedded_graphics::prelude::Primitive;
use embedded_graphics::Drawable;

pub struct Controller {
    model: (),
    view: (),
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller {
    #[must_use]
    pub fn new() -> Self {
        Self {
            model: (),
            view: ()
        }
    }

    pub fn run<T>(&mut self, touchscreen: &mut T)
    where
        T: Touchscreen,
        <T as DrawTarget>::Error: Debug,
        T: DrawTarget<Color = Rgb888>,
    {
        if let Some(TouchEvent { x, y, r#type }) = touchscreen.get_touch_event() {
            let fill = PrimitiveStyle::with_fill(Rgb888::new(255, 0, 0));
            Rectangle::with_center(Point::new(x.into(), y.into()), Size::new(16, 16))
                .into_styled(fill)
                .draw(touchscreen).unwrap();
        }
    }
}
