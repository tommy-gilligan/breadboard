use application::touchscreen::{TouchEvent, TouchEventType, Touchscreen};
use embedded_graphics_core::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, Size},
    Pixel,
};
use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use std::error::Error;
use std::sync::mpsc::{channel, Receiver, Sender};
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

enum MouseEventType {
    Down,
    Move,
    Up,
    Leave,
}

struct MouseEvent {
    x: i32,
    y: i32,
    r#type: MouseEventType,
}

pub struct Web {
    simulator_display: WebSimulatorDisplay<Rgb888>,
    down: bool,
    channel: (Sender<MouseEvent>, Receiver<MouseEvent>),
}

impl Web {
    pub fn new(element: &Element) -> Self {
        let simulator_display = WebSimulatorDisplay::new(
            (480, 320),
            &OutputSettingsBuilder::new()
                .scale(1)
                .pixel_spacing(0)
                .build(),
            Some(element),
        );

        let html_element = element.dyn_ref::<HtmlElement>().unwrap();

        let result = Self {
            simulator_display,
            down: false,
            channel: channel(),
        };

        {
            let sender = result.channel.0.clone();
            let mousedown_closure =
                Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                    sender
                        .send(MouseEvent {
                            x: event.offset_x(),
                            y: event.offset_y(),
                            r#type: MouseEventType::Down,
                        })
                        .unwrap();
                });

            html_element
                .add_event_listener_with_callback(
                    "mousedown",
                    mousedown_closure.as_ref().unchecked_ref(),
                )
                .unwrap();

            mousedown_closure.forget();
        }

        {
            let sender = result.channel.0.clone();
            let mousemove_closure =
                Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                    sender
                        .send(MouseEvent {
                            x: event.offset_x(),
                            y: event.offset_y(),
                            r#type: MouseEventType::Move,
                        })
                        .unwrap();
                });

            html_element
                .add_event_listener_with_callback(
                    "mousemove",
                    mousemove_closure.as_ref().unchecked_ref(),
                )
                .unwrap();

            mousemove_closure.forget();
        }

        {
            let sender = result.channel.0.clone();
            let mouseup_closure =
                Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                    sender
                        .send(MouseEvent {
                            x: event.offset_x(),
                            y: event.offset_y(),
                            r#type: MouseEventType::Up,
                        })
                        .unwrap();
                });

            html_element
                .add_event_listener_with_callback(
                    "mouseup",
                    mouseup_closure.as_ref().unchecked_ref(),
                )
                .unwrap();

            mouseup_closure.forget();
        }

        {
            let sender = result.channel.0.clone();
            let mouseleave_closure =
                Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                    sender
                        .send(MouseEvent {
                            x: event.offset_x(),
                            y: event.offset_y(),
                            r#type: MouseEventType::Leave,
                        })
                        .unwrap();
                });

            html_element
                .add_event_listener_with_callback(
                    "mouseleave",
                    mouseleave_closure.as_ref().unchecked_ref(),
                )
                .unwrap();

            mouseleave_closure.forget();
        }

        result
    }

    pub fn flush(&mut self) {
        self.simulator_display.flush().unwrap();
    }
}

impl DrawTarget for Web {
    type Color = Rgb888;
    type Error = Box<dyn Error>;

    fn draw_iter<I: IntoIterator<Item = Pixel<<Self as DrawTarget>::Color>>>(
        &mut self,
        i: I,
    ) -> Result<(), <Self as DrawTarget>::Error> {
        let result = self.simulator_display.draw_iter(i);
        self.simulator_display.flush().unwrap();
        result
    }
}

impl OriginDimensions for Web {
    fn size(&self) -> Size {
        Size::new(480, 320)
    }
}

impl Touchscreen for Web {
    fn get_touch_event(&mut self) -> Option<TouchEvent> {
        match self.channel.1.try_recv() {
            Ok(MouseEvent {
                x,
                y,
                r#type: MouseEventType::Down,
            }) => {
                self.down = true;
                Some(TouchEvent {
                    x: x as u16,
                    y: y as u16,
                    r#type: TouchEventType::Start,
                })
            }
            Ok(MouseEvent {
                x,
                y,
                r#type: MouseEventType::Up | MouseEventType::Leave,
            }) => {
                if self.down {
                    self.down = false;
                    Some(TouchEvent {
                        x: x as u16,
                        y: y as u16,
                        r#type: TouchEventType::End,
                    })
                } else {
                    None
                }
            }
            Ok(MouseEvent {
                x,
                y,
                r#type: MouseEventType::Move,
            }) => {
                if self.down {
                    Some(TouchEvent {
                        x: x as u16,
                        y: y as u16,
                        r#type: TouchEventType::Move,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
