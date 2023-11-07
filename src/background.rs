use embedded_graphics::{
    geometry::{OriginDimensions, Size},
    pixelcolor::Rgb888,
    prelude::{Point, Primitive},
    primitives::{PrimitiveStyleBuilder, Rectangle},
    Drawable,
};

use embedded_graphics_core::draw_target::DrawTarget;

use core::fmt::Debug;

const VERTICAL_OFFSET: i32 = 51;
const POINT_SIZE: i32 = 18;

pub struct View;

impl View {
    pub fn draw<DT, E>(&mut self, display: &mut DT)
    where
        E: Debug,
        DT: DrawTarget<Color = Rgb888, Error = E> + OriginDimensions,
    {
        for index in 0..25 {
            let fill = PrimitiveStyleBuilder::new()
                .fill_color(if index % 2 == 0 {
                    Rgb888::new(0, 0, 0)
                } else {
                    Rgb888::new(40, 40, 40)
                })
                .build();

            Rectangle::new(
                Point::new(index * POINT_SIZE, 0),
                Size::new(
                    POINT_SIZE.try_into().unwrap(),
                    VERTICAL_OFFSET as u32,
                ),
            )
            .into_styled(fill)
            .draw(display)
            .unwrap();

            let fill = PrimitiveStyleBuilder::new()
                .fill_color(if index % 2 == 0 {
                    Rgb888::new(0, 0, 0)
                } else {
                    Rgb888::new(255, 255, 255)
                })
                .build();
            Rectangle::new(
                Point::new(index * POINT_SIZE, VERTICAL_OFFSET),
                Size::new(
                    POINT_SIZE.try_into().unwrap(),
                    5 * POINT_SIZE as u32,
                ),
            )
            .into_styled(fill)
            .draw(display)
            .unwrap();
        }
    }
}
