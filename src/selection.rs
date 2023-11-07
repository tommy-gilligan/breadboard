use embedded_graphics::{
    geometry::OriginDimensions,
    pixelcolor::Rgb888,
    prelude::{Point, Primitive},
    primitives::{PrimitiveStyleBuilder, Rectangle},
    Drawable,
};

use embedded_graphics_core::draw_target::DrawTarget;

use core::fmt::Debug;

const POINT_SIZE: i32 = 18;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectionEventType {
    Start,
    Update,
    End,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelectionEvent {
    pub column: usize,
    pub r#type: SelectionEventType,
}

pub struct View;
pub struct ViewModel(pub Option<usize>, pub Option<usize>);

impl View {
    pub fn draw<DT, E>(&mut self, display: &mut DT, view_model: ViewModel)
    where
        E: Debug,
        DT: DrawTarget<Color = Rgb888, Error = E> + OriginDimensions,
    {
        let fill = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::new(255, 0, 0))
            .build();

        if let Some(column) = view_model.0 {
            let destination_index = if let Some(end_column) = view_model.1 {
                end_column
            } else {
                column
            };

            let left = column.min(destination_index) as i32;
            let right = column.max(destination_index) as i32;

            Rectangle::with_corners(
                Point::new(left * POINT_SIZE, 0),
                Point::new((right + 1) * POINT_SIZE, POINT_SIZE),
            )
            .into_styled(fill)
            .draw(display)
            .unwrap();
        }
    }
}
