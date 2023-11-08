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
use touchscreen::{TouchEvent, Touchscreen};

pub struct Controller {
    needs_redraw: bool
}

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

impl Controller {
    pub fn new() -> Self {
        Self {
            needs_redraw: true
        }
    }

    pub fn tick<T>(&mut self, touchscreen: &mut T) where T: Touchscreen, <T as DrawTarget>::Error: Debug, T: DrawTarget<Color = Rgb888> {
        if self.needs_redraw {
            self.needs_redraw = false;
            let n_colors = COLORS.len();
            let color_width: u32 = touchscreen.size().width / n_colors as u32;
            let mut x_offset = 0;

            for color in COLORS {
                let fill = PrimitiveStyle::with_fill(color);
                Rectangle::new(Point::new(x_offset, 0), Size::new(color_width, touchscreen.size().height))
                    .into_styled(fill)
                    .draw(touchscreen)
                    .unwrap();
                x_offset += color_width as i32;
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
mod web {
    extern crate std;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .expect("no global `window` exists")
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    #[wasm_bindgen(start)]
    fn run() {
        let touchscreen_div = web_sys::window()
            .expect("no global `window` exists")
            .document()
            .unwrap()
            .get_element_by_id("touchscreen")
            .unwrap();
        let mut web_touchscreen = web::Web::new(&touchscreen_div);

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
