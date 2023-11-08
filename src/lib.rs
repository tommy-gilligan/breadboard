#![no_std]
use core::fmt::Debug;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::DrawTarget;

use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::Drawable;
use embedded_graphics_core::{
    prelude::{Point, Size},
    primitives::Rectangle,
};
use touchscreen::Touchscreen;

const COLORS: [Rgb888; 8] = [
    Rgb888::new(255, 0, 0),
    Rgb888::new(0, 255, 0),
    Rgb888::new(0, 0, 255),
    Rgb888::new(255, 0, 255),
    Rgb888::new(0, 255, 255),
    Rgb888::new(255, 255, 0),
    Rgb888::new(0, 0, 0),
    Rgb888::new(255, 255, 255),
];

pub struct Controller {
    selected_color: Option<Rgb888>,
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
            selected_color: None,
        }
    }

    pub fn tick<T>(&mut self, touchscreen: &mut T)
    where
        T: Touchscreen,
        <T as DrawTarget>::Error: Debug,
        T: DrawTarget<Color = Rgb888>,
    {
        let mut x_offset = 0;

        if let Some(_selected_color) = self.selected_color {
            if let Ok(Some(touchscreen::TouchEvent { r#type, .. })) = touchscreen.get_touch_event()
            {
                if r#type == touchscreen::TouchEventType::End {
                    self.selected_color = None;

                    let color_width = touchscreen.size().width / COLORS.len() as u32;

                    for color in COLORS {
                        let fill = PrimitiveStyle::with_fill(color);
                        Rectangle::new(
                            Point::new(x_offset, 0),
                            Size::new(color_width, touchscreen.size().height),
                        )
                        .into_styled(fill)
                        .draw(touchscreen)
                        .unwrap();
                        x_offset += color_width as i32;
                    }
                }
            }
        } else if let Ok(Some(touchscreen::TouchEvent { x, r#type, .. })) =
            touchscreen.get_touch_event()
        {
            if r#type == touchscreen::TouchEventType::End {
                let color_width = touchscreen.size().width / COLORS.len() as u32;

                self.selected_color = Some(COLORS[x as usize / color_width as usize]);

                let fill = PrimitiveStyle::with_fill(self.selected_color.unwrap());
                Rectangle::new(
                    Point::new(0, 0),
                    Size::new(touchscreen.size().width, touchscreen.size().height),
                )
                .into_styled(fill)
                .draw(touchscreen)
                .unwrap();
            }
        }
    }
}

#[cfg(target_family = "wasm")]
mod web {
    extern crate std;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .unwrap();
    }

    #[wasm_bindgen(start)]
    fn run() {
        let touchscreen_div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("touchscreen")
            .unwrap();

        let mut web_touchscreen = touchscreen::web_screen::Web::new(
            (480, 320),
            &touchscreen::web_screen::OutputSettingsBuilder::new()
                .scale(1)
                .pixel_spacing(0)
                .build(),
            &touchscreen_div,
        );

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut controller = super::Controller::new();

        *g.borrow_mut() = Some(Closure::new(move || {
            controller.tick(&mut web_touchscreen);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
