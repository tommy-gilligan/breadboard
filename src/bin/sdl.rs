extern crate embedded_graphics;

use application::Controller;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
use touchscreen::Touchscreen;

use std::{thread, time};

pub fn main() {
    let display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(800, 480));
    let window = Window::new("Click to move circle", &OutputSettings::default());

    let mut touchscreen = touchscreen::sdl_screen::Sdl::new(display, window);
    let mut controller = Controller::new();
    controller.tick(&mut touchscreen);
    touchscreen.get_touch_event().unwrap();

    loop {
        controller.tick(&mut touchscreen);
        touchscreen.get_touch_event().unwrap();
        thread::sleep(time::Duration::from_millis(10));
    }
}
