use core::fmt::Debug;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::DrawTarget;

use crate::model::Connection;
use crate::touchscreen::{TouchEvent, TouchEventType, Touchscreen};
use crate::view::{Breadboard, HitTestResult, Region, SelectionEvent, SelectionEventType};

pub struct Controller {
    connections: crate::model::Connections,
    view: crate::view::Breadboard,
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller {
    #[must_use] pub fn new() -> Self {
        Self {
            connections: crate::model::Connections::new(),
            view: Breadboard::new((18, 18)),
        }
    }

    pub fn redraw<T>(&mut self, touchscreen: &mut T)
    where
        T: Touchscreen,
        <T as DrawTarget>::Error: Debug,
        T: DrawTarget<Color = Rgb888>,
    {
        self.view.draw(touchscreen, &self.connections);
    }

    pub fn run<T>(&mut self, touchscreen: &mut T)
    where
        T: Touchscreen,
        <T as DrawTarget>::Error: Debug,
        T: DrawTarget<Color = Rgb888>,
    {
        if let Some(TouchEvent { x, y, r#type }) = touchscreen.get_touch_event() {
            let HitTestResult::HitColumn((region, column)) = self.view.hit_test(i32::from(x), i32::from(y));
            let selection_event_type = match r#type {
                TouchEventType::Start => SelectionEventType::Start,
                TouchEventType::Move => SelectionEventType::Update,
                TouchEventType::End => SelectionEventType::End,
            };
            let selection = self.view.update_selection(SelectionEvent {
                region,
                column,
                r#type: selection_event_type,
            });

            match r#type {
                TouchEventType::Start => {
                    self.view.draw(touchscreen, &self.connections);
                }
                TouchEventType::End => {
                    let (a, b) = selection.unwrap();

                    if region == Region::Top {
                        self.connections.insert(Connection::Top(a, b + 1));
                    } else {
                        self.connections.insert(Connection::Bottom(a, b + 1));
                    }
                    self.view.draw(touchscreen, &self.connections);
                }
                TouchEventType::Move => {
                    self.view.draw_selection_highlight(touchscreen);
                }
            };
        }
    }
}
