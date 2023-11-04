use embedded_graphics_core::prelude::{DrawTarget, OriginDimensions};

#[derive(Debug)]
pub enum TouchEventType {
    Start,
    Move,
    End,
}

#[derive(Debug)]
pub struct TouchEvent {
    pub x: u16,
    pub y: u16,
    pub r#type: TouchEventType,
}

pub trait Touchscreen: DrawTarget + OriginDimensions {
    fn get_touch_event(&mut self) -> Option<TouchEvent>;
}
