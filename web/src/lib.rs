use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
mod touchscreen;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let touchscreen_div = window()
        .document()
        .unwrap()
        .get_element_by_id("touchscreen")
        .unwrap();
    let mut web_touchscreen = touchscreen::Web::new(&touchscreen_div);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut controller = application::controller::Controller::new();
    controller.redraw(&mut web_touchscreen);
    *g.borrow_mut() = Some(Closure::new(move || {
        controller.run(&mut web_touchscreen);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
