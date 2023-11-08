#![no_std]
use embedded_graphics_core::prelude::{DrawTarget, OriginDimensions};

#[derive(Debug)]
pub enum TouchEventType {
    Start,
    Move,
    End,
}

#[derive(Debug)]
pub struct TouchEvent {
    pub x: i32,
    pub y: i32,
    pub r#type: TouchEventType,
}

pub trait Touchscreen: DrawTarget + OriginDimensions {
    fn get_touch_event(&mut self) -> Option<TouchEvent>;
}

#[cfg(feature = "red_screen")]
pub mod red_screen;

#[cfg(feature = "web_screen")]
pub mod web_screen;
