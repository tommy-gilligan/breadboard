use embedded_graphics::{
    geometry::OriginDimensions,
    pixelcolor::Rgb888,
};
use embedded_graphics_core::draw_target::DrawTarget;
use core::fmt::Debug;
use crate::model;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HitTestResult {
    HitColumn(usize),
}

pub struct Breadboard;

impl Breadboard {
    pub fn draw<DT, E>(&mut self, display: &mut DT, model: &model::Connections)
    where
        E: Debug,
        DT: DrawTarget<Color = Rgb888, Error = E> + OriginDimensions,
    {
        crate::background::View.draw(display);
        crate::selection::View.draw(display, crate::selection::ViewModel(None, None));
    }
}
