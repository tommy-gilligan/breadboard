#![no_std]
use core::fmt::Debug;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::DrawTarget;

use touchscreen::{TouchEvent, Touchscreen};
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics_core::{prelude::{Point, Size}, primitives::Rectangle};
use embedded_graphics::prelude::Primitive;
use embedded_graphics::Drawable;

pub struct Controller;

impl Controller {
    pub fn tick<T>(&mut self, touchscreen: &mut T)
    where
        T: Touchscreen,
        <T as DrawTarget>::Error: Debug,
        T: DrawTarget<Color = Rgb888>,
    {
        if let Some(TouchEvent { x, y, .. }) = touchscreen.get_touch_event() {
            let fill = PrimitiveStyle::with_fill(Rgb888::new(255, 0, 0));
            Rectangle::with_center(Point::new(x.into(), y.into()), Size::new(16, 16))
                .into_styled(fill)
                .draw(touchscreen).unwrap();
        }
    }
}

#[cfg(not(target_os="none"))]
mod web {
    extern crate std;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window().expect("no global `window` exists")
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    #[wasm_bindgen(start)]
    fn run() {
        let touchscreen_div = web_sys::window().expect("no global `window` exists")
            .document()
            .unwrap()
            .get_element_by_id("touchscreen")
            .unwrap();
        let mut web_touchscreen = web::Web::new(&touchscreen_div);

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut controller = super::Controller;
        controller.tick(&mut web_touchscreen);
        *g.borrow_mut() = Some(Closure::new(move || {
            controller.tick(&mut web_touchscreen);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
