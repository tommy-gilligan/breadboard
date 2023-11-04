use embedded_graphics::{
    pixelcolor::PixelColor,
    prelude::*,
    primitives::{Rectangle, PrimitiveStyleBuilder, StrokeAlignment},
};
use heapless::FnvIndexSet;
use core::convert::TryInto;

use embedded_graphics::prelude::Point;

pub fn draw<const COLUMNS: usize, const ROWS: usize, T>(
    connections: &mut Connections,
    connection_area: &mut ConnectionArea::<COLUMNS, ROWS, T>,
    colors: &[T]
) where T: PixelColor + Default + core::fmt::Debug + core::cmp::Eq + core::hash::Hash {
    for (&(from, to), (row, color)) in connections.iter()
        .zip((0..ROWS).cycle().zip(colors.iter().cycle())) {
            for column in from..to {
                let connection_point = connection_area.connection_point((row, column));

                connection_point.border_width = 0;
                connection_point.color = *color;
            }
    }
}

pub struct Connections(FnvIndexSet::<(usize, usize), 16>);

impl Connections {
    pub fn new() -> Self {
        Self(FnvIndexSet::new())
    }

    pub fn add(&mut self, connection: (usize, usize)) {
        let connection = (
            connection.0.min(connection.1),
            connection.0.max(connection.1)
        );
        let _ = self.0.insert(connection);
    }

    pub fn toggle(&mut self, connection: (usize, usize)) {
        let connection = (
            connection.0.min(connection.1),
            connection.0.max(connection.1)
        );

        if self.0.contains(&connection) {
            self.0.remove(&connection);
        } else {
            let _ = self.0.insert(connection);
        }
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &(usize, usize)> {
        self.0.iter()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConnectionPoint<T: PixelColor + Default + core::fmt::Debug + core::cmp::Eq + core::hash::Hash> {
  pub p: Point,
  pub q: Point,
  pub border_color: T,
  pub color: T,
  pub border_width: u32
}

impl <T: PixelColor + Default + core::fmt::Debug + core::cmp::Eq + core::hash::Hash>ConnectionPoint<T> {
    pub fn new(p: Point, q: Point) -> Self {
        Self {
          p, q,
          border_color: Default::default(),
          color: Default::default(),
          border_width: Default::default()
        }
    }
}

#[derive(Clone, Debug)]
struct Row<const COLUMNS: usize, T: PixelColor + Default + core::fmt::Debug + core::cmp::Eq + core::hash::Hash> {
  columns: [ConnectionPoint<T>; COLUMNS]
}

impl <const COLUMNS: usize, T: PixelColor + Default + core::fmt::Debug + core::cmp::Eq + core::hash::Hash>Row<COLUMNS, T> {
    pub fn new(point_width: i32, y1: i32, y2: i32) -> Self {
        let mut acc = 0;
        Self {
            columns: (0..COLUMNS).map(|_| {
                let point = ConnectionPoint::new(Point::new(acc, y1), Point::new(acc + point_width, y2));
                acc += point_width;
                point
            }).collect::<Vec<ConnectionPoint<T>>>().try_into().unwrap()
        }
    }
}

#[derive(Clone)]
pub struct ConnectionArea<const COLUMNS: usize, const ROWS: usize, T: PixelColor + Default + core::fmt::Debug + core::cmp::Eq + core::hash::Hash> {
  rows: [Row<COLUMNS, T>; ROWS],
}

impl <const COLUMNS: usize, const ROWS: usize, T: PixelColor + Default + core::fmt::Debug + core::cmp::Eq + core::hash::Hash>ConnectionArea<COLUMNS, ROWS, T> {
    pub fn new(point_width: i32, point_height: i32) -> Self {
        let mut acc = 0;
        let rows: [Row<COLUMNS, T>; ROWS] = (0..ROWS).map(|_| {
            let row = Row::new(point_width, acc, acc + point_height);
            acc += point_height;
            row
        }).collect::<Vec<Row<COLUMNS, T>>>().try_into().unwrap();

        Self { rows }
    }

    pub fn connection_point(&mut self, point: (usize, usize)) -> &mut ConnectionPoint<T> {
        &mut self.rows[point.0].columns[point.1]
    }

    pub fn draw<D>(&self, display: &mut D) where D: embedded_graphics::draw_target::DrawTarget<Color = T>, <D as DrawTarget>::Error: core::fmt::Debug {
        for row in self.rows.iter() {
            for point in row.columns.iter() {
                let style = PrimitiveStyleBuilder::new()
                    .stroke_color(point.border_color)
                    .stroke_width(point.border_width)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .fill_color(point.color)
                    .build();

                Rectangle::with_corners(point.p, point.q)
                    .into_styled(style)
                    .draw(display)
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::pixelcolor::Rgb555;

    #[test]
    fn test_connection_area() {
        let _connection_area = ConnectionArea::<30, 5, Rgb555>::new(5, 5);
    }

    #[test]
    fn test_draw() {
        let mut connection_area = ConnectionArea::<2, 2, Rgb555>::new(2, 3);
        let connection_point = connection_area.connection_point((0, 0));
        connection_point.p = Point::new(0, 0);
        connection_point.q = Point::new(10, 10);
        connection_point.color = Rgb555::new(25, 75, 150);
        connection_point.border_color = Rgb555::new(55, 26, 161);
        connection_point.border_width = 2;

        let connection_point = connection_area.connection_point((0, 1));
        connection_point.p = Point::new(11, 0);
        connection_point.q = Point::new(13, 3);
        connection_point.color = Rgb555::new(35, 55, 50);
        connection_point.border_color = Rgb555::new(45, 27, 163);
        connection_point.border_width = 1;

        let connection_point = connection_area.connection_point((1, 0));
        connection_point.p = Point::new(0, 11);
        connection_point.q = Point::new(7, 13);
        connection_point.color = Rgb555::new(15, 25, 10);
        connection_point.border_color = Rgb555::new(26, 55, 161);
        connection_point.border_width = 1;

        let connection_point = connection_area.connection_point((1, 1));
        connection_point.p = Point::new(12, 12);
        connection_point.q = Point::new(13, 13);
        connection_point.color = Rgb555::new(35, 55, 50);
        connection_point.border_color = Rgb555::new(45, 27, 163);
        connection_point.border_width = 1;

        let mut display = MockDisplay::new();
        connection_area.draw(&mut display);

        let mut expected_display = MockDisplay::new();

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb555::new(55, 26, 161))
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Inside)
            .fill_color(Rgb555::new(25, 75, 150))
            .build();

        Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10))
            .into_styled(style)
            .draw(&mut expected_display)
            .unwrap();

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb555::new(45, 27, 163))
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .fill_color(Rgb555::new(35, 55, 50))
            .build();

        Rectangle::with_corners(Point::new(11, 0), Point::new(13, 3))
            .into_styled(style)
            .draw(&mut expected_display)
            .unwrap();

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb555::new(26, 55, 161))
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .fill_color(Rgb555::new(15, 25, 10))
            .build();

        Rectangle::with_corners(Point::new(0, 11), Point::new(7, 13))
            .into_styled(style)
            .draw(&mut expected_display)
            .unwrap();

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb555::new(45, 27, 163))
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .fill_color(Rgb555::new(35, 55, 50))
            .build();

        Rectangle::with_corners(Point::new(12, 12), Point::new(13, 13))
            .into_styled(style)
            .draw(&mut expected_display)
            .unwrap();

        display.assert_eq(&expected_display);
    }

    #[test]
    fn test_connection_point() {
        let mut connection_area = ConnectionArea::<30, 5, Rgb555>::new(5, 5);
        let connection_point = connection_area.connection_point((2, 1));
        connection_point.p = Point::new(0, 0);
        connection_point.q = Point::new(10, 10);
        connection_point.color = Rgb555::new(25, 75, 150);
        connection_point.border_color = Rgb555::new(55, 26, 161);
        connection_point.border_width = 2;

        assert_eq!(
            connection_area.connection_point((2, 1)),
            &mut ConnectionPoint {
              p: Point::new(0, 0),
              q: Point::new(10, 10),
              border_color: Rgb555::new(55, 26, 161),
              color: Rgb555::new(25, 75, 150),
              border_width: 2
            }
        )
    }
}
