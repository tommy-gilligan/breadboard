use embedded_graphics_web_simulator::display::WebSimulatorDisplay;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::{OriginDimensions, Size};
use embedded_graphics::mono_font::{ascii::FONT_9X15_BOLD, MonoTextStyle};
use embedded_graphics::pixelcolor::{PixelColor, Rgb565};
use embedded_graphics::prelude::{Point, Primitive};
use embedded_graphics::primitives::{Rectangle, PrimitiveStyleBuilder, PrimitiveStyle, StrokeAlignment, Line};
use embedded_graphics::text::{Text, Baseline, Alignment, LineHeight, TextStyleBuilder};
use std::collections::HashSet;
use web_sys::console;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Region {
    Top,
    Bottom
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HitTestResult<'a> {
    Miss,
    HitPoint((&'a str, &'a str)),
    HitColumnLabel((Region, String))
}

#[derive(Clone, Debug)]
pub struct Row<T: PixelColor + Default + core::fmt::Debug + core::cmp::Eq + core::hash::Hash> {
  pub columns: [T; 3]
}

pub struct Breadboard<'a> {
    pub margin: (i32, i32, i32, i32),
    pub trench: i32,
    pub display: Option<WebSimulatorDisplay<Rgb565>>,
    pub rows: [Row<Rgb565>; 4],
    pub point_size: (i32, i32),
    pub row_labels: Vec<&'a str>,
    pub column_labels: Vec<&'a str>,
    pub selected_column_labels: HashSet<(Region, String)>
}

impl <'a>Breadboard<'a> {
    pub fn new(margin: (i32, i32, i32, i32), trench: i32, point_size: (i32, i32)) -> Self {
        Self {
            margin,
            trench,
            display: None,
            selected_column_labels: HashSet::new(),
            point_size,
            row_labels: ["A", "B", "C", "D"].to_vec(),
            column_labels: ["1", "2", "3"].to_vec(),
            rows: [
                Row { columns: [Rgb565::new(255, 0, 255), Rgb565::new(255, 0, 255), Rgb565::new(255, 0, 255)] },
                Row { columns: [Rgb565::new(255, 0, 255), Rgb565::new(255, 0, 255), Rgb565::new(255, 0, 255)] },
                Row { columns: [Rgb565::new(255, 0, 255), Rgb565::new(255, 0, 255), Rgb565::new(255, 0, 255)] },
                Row { columns: [Rgb565::new(255, 0, 255), Rgb565::new(255, 0, 255), Rgb565::new(255, 0, 255)] },
            ]
        }
    }

    pub fn init(&mut self, display: WebSimulatorDisplay<Rgb565>) {
        self.display = Some(display);
    }

    fn draw_selection_highlight(&mut self) {
        let fill = PrimitiveStyleBuilder::new().fill_color(Rgb565::new(255, 255, 0)).build();
        for (region, column_label) in self.selected_column_labels.iter() {
            let index = self.column_labels.iter().position(|cl| column_label == cl).unwrap() as i32;

            if region == &Region::Top {
                Rectangle::new(
                    Point::new(
                        self.margin.3 + index * self.point_size.0,
                        0
                    ),
                    Size::new(
                        self.point_size.0.try_into().unwrap(),
                        self.margin.0.try_into().unwrap()
                    )
                )
                .into_styled(fill)
                .draw(self.display.as_mut().unwrap())
                .unwrap();
            } else {
                Rectangle::new(
                    Point::new(
                        self.margin.3 + index * self.point_size.0,
                        self.margin.0 + self.point_size.1 * 4 as i32 + self.trench
                    ),
                    Size::new(
                        self.point_size.0.try_into().unwrap(),
                        self.margin.2.try_into().unwrap()
                    )
                )
                .into_styled(fill)
                .draw(self.display.as_mut().unwrap())
                .unwrap();
            };
        }
    }

    fn draw_points(&mut self) {
        for (row_index, row) in self.rows.iter().enumerate() {
            for (column_index, color) in row.columns.iter().enumerate() {
                let trench = if row_index >= 4 / 2 {
                    self.trench
                } else {
                    0
                };
                let fill = PrimitiveStyleBuilder::new().fill_color(*color).build();
                Rectangle::new(
                    Point::new(
                        self.margin.3 + column_index as i32 * self.point_size.0,
                        self.margin.0 + trench + row_index as i32 * self.point_size.1
                    ),
                    Size::new(
                        self.point_size.0.try_into().unwrap(),
                        self.point_size.1.try_into().unwrap()
                    )
                )
                .into_styled(fill)
                .draw(self.display.as_mut().unwrap())
                .unwrap();
            }
        }
    }

    fn draw_borders(&mut self) {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::new(0, 0, 0))
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();

        let size = self.display.as_ref().unwrap().size();

        // top
        Line::new(
            Point::new(0, self.margin.0),
            Point::new(size.width.try_into().unwrap(), self.margin.0)
        ).into_styled(style).draw(self.display.as_mut().unwrap()).unwrap();
        // bottom
        Line::new(
            Point::new(0, self.margin.0 + self.trench + self.point_size.1 * 4 as i32),
            Point::new(size.width.try_into().unwrap(), self.margin.0 + self.trench + self.point_size.1 * 4 as i32)
        ).into_styled(style).draw(self.display.as_mut().unwrap()).unwrap();

        // left
        Line::new(
            Point::new(self.margin.3, 0),
            Point::new(self.margin.3, size.height as i32)
        ).into_styled(style).draw(self.display.as_mut().unwrap()).unwrap();
        // right
        Line::new(
            Point::new(self.margin.3 + self.point_size.0 * 3 as i32, 0),
            Point::new(self.margin.3 + self.point_size.0 * 3 as i32, size.height as i32)
        ).into_styled(style).draw(self.display.as_mut().unwrap()).unwrap();
    }

    fn draw_labels(&mut self) {
        let character_style = MonoTextStyle::new(
            &FONT_9X15_BOLD,
            Rgb565::new(0, 0, 0)
        );

        for (index, label) in self.column_labels.iter().enumerate() {
            let text_style = TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .baseline(Baseline::Top)
                .build();
            Text::with_text_style(
                label,
                Point::new(self.margin.3 as i32 + index as i32 * self.point_size.0 + self.point_size.0 / 2, 0),
                character_style,
                text_style,
            )
            .draw(self.display.as_mut().unwrap()).unwrap();

            let text_style = TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .baseline(Baseline::Bottom)
                .build();
            Text::with_text_style(
                label,
                Point::new(self.margin.3 as i32 + index as i32 * self.point_size.0 + self.point_size.0 / 2, self.margin.0 + self.trench + 4 as i32 * self.point_size.1 + self.margin.2),
                character_style,
                text_style,
            )
            .draw(self.display.as_mut().unwrap()).unwrap();
        }

        for (index, label) in self.row_labels.iter().enumerate() {
            let text_style = TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Middle)
                .build();
            let trench = if index >= 4 / 2 {
                self.trench
            } else {
                0
            };
            Text::with_text_style(
                label,
                Point::new(0, self.margin.0 + self.point_size.1 * index as i32 + self.point_size.1 / 2 + trench),
                character_style,
                text_style,
            )
            .draw(self.display.as_mut().unwrap()).unwrap();

            let text_style = TextStyleBuilder::new()
                .alignment(Alignment::Right)
                .baseline(Baseline::Middle)
                .build();
            Text::with_text_style(
                label,
                Point::new(self.margin.3 + 3 as i32 * self.point_size.0 + self.margin.0, self.margin.0 + self.point_size.1 * index as i32 + self.point_size.1 / 2 + trench),
                character_style,
                text_style,
            )
            .draw(self.display.as_mut().unwrap()).unwrap();
        }
    }

    fn hit_column(&self, x: i32, _y: i32) -> bool {
        x > self.margin.3 && (x < self.margin.3 + self.point_size.0 * 3 as i32)
    }

    fn hit_row(&self, _x: i32, y: i32) -> bool {
        (y > self.margin.0 && (y < self.margin.0 + self.point_size.1 * 4 as i32 / 2)) ||
        (
          y > (self.margin.0 + self.point_size.1 * 4 as i32 / 2 + self.trench)
          && (y < self.margin.0 + self.point_size.1 * 4 as i32 + self.trench)
        )
    }

    pub fn draw(&mut self) {
        let fill = PrimitiveStyleBuilder::new().fill_color(Rgb565::new(255, 255, 255)).build();
        Rectangle::new(Point::new(0, 0), self.display.as_mut().unwrap().size())
            .into_styled(fill)
            .draw(self.display.as_mut().unwrap())
            .unwrap();

        self.draw_points();
        self.draw_selection_highlight();
        self.draw_borders();
        self.draw_labels();

        self.display.as_mut().unwrap().flush().expect("Couldn't update");
    }

    pub fn select_column_label(&mut self, region: Region, column: String) {
        if self.selected_column_labels.contains(&(region.clone(), column.clone())) {
            self.selected_column_labels.remove(&(region, column));
        } else {
            self.selected_column_labels.insert((region, column));
        }
    }

    pub fn hit_test(&self, x: i32, y: i32) -> HitTestResult {
        if self.hit_column(x, y) {
            if self.hit_row(x, y) {
                let trench = if y > self.margin.0 + 4 as i32 / 2 * self.point_size.1 {
                    self.trench
                } else {
                    0
                };

                HitTestResult::HitPoint(
                    (
                        self.row_labels[
                            ((y - self.margin.0 - trench) / self.point_size.1) as usize
                        ],
                        self.column_labels[
                            ((x - self.margin.3) / self.point_size.0) as usize
                        ]
                    )
                )
            } else {
                let region = if y > self.margin.0 + 4 as i32 / 2 * self.point_size.1 {
                    Region::Bottom
                } else {
                    Region::Top
                };
                HitTestResult::HitColumnLabel(
                    (
                        region,
                        self.column_labels[
                            ((x - self.margin.3) / self.point_size.0) as usize
                        ].to_string()
                    )
                )
            }
        } else {
            HitTestResult::Miss
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::pixelcolor::Rgb555;

    #[test]
    fn test_init() {
        let mut breadboard = Breadboard {
            margin: (0, 0, 0, 0),
            trench: 0,
            display: None,
            rows: [
                Row { columns: [Rgb565::default()] },
                Row { columns: [Rgb565::default()] },
            ],
            point_size: (2, 2),
            row_labels: ["A", "B"],
            column_labels: ["1"],
        };
        let display: MockDisplay<Rgb565> = MockDisplay::new();
        breadboard.init(display);
    }

    #[test]
    fn test_click_on_point() {
        let mut breadboard: Breadboard<2, 1> = Breadboard {
            margin: (5, 7, 5, 7),
            trench: 3,
            display: None,
            rows: [
                Row { columns: [Rgb565::new(255, 0, 0)] },
                Row { columns: [Rgb565::new(0, 255, 0)] },
            ],
            point_size: (2, 2),
            row_labels: ["A", "B"],
            column_labels: ["1"],
        };
    }

    // #[test]
    // fn test_draw() {
    //     let connection_point = connection_area.connection_point((1, 1));
    //     connection_point.p = Point::new(12, 12);
    //     connection_point.q = Point::new(13, 13);
    //     connection_point.color = Rgb555::new(35, 55, 50);
    //     connection_point.border_color = Rgb555::new(45, 27, 163);
    //     connection_point.border_width = 1;

    //     let mut display = MockDisplay::new();
    //     connection_area.draw(&mut display);

    //     let mut expected_display = MockDisplay::new();

    //     let style = PrimitiveStyleBuilder::new()
    //         .stroke_color(Rgb555::new(55, 26, 161))
    //         .stroke_width(2)
    //         .stroke_alignment(StrokeAlignment::Inside)
    //         .fill_color(Rgb555::new(25, 75, 150))
    //         .build();

    //     Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10))
    //         .into_styled(style)
    //         .draw(&mut expected_display)
    //         .unwrap();

    //     let style = PrimitiveStyleBuilder::new()
    //         .stroke_color(Rgb555::new(45, 27, 163))
    //         .stroke_width(1)
    //         .stroke_alignment(StrokeAlignment::Inside)
    //         .fill_color(Rgb555::new(35, 55, 50))
    //         .build();

    //     Rectangle::with_corners(Point::new(11, 0), Point::new(13, 3))
    //         .into_styled(style)
    //         .draw(&mut expected_display)
    //         .unwrap();

    //     let style = PrimitiveStyleBuilder::new()
    //         .stroke_color(Rgb555::new(26, 55, 161))
    //         .stroke_width(1)
    //         .stroke_alignment(StrokeAlignment::Inside)
    //         .fill_color(Rgb555::new(15, 25, 10))
    //         .build();

    //     Rectangle::with_corners(Point::new(0, 11), Point::new(7, 13))
    //         .into_styled(style)
    //         .draw(&mut expected_display)
    //         .unwrap();

    //     let style = PrimitiveStyleBuilder::new()
    //         .stroke_color(Rgb555::new(45, 27, 163))
    //         .stroke_width(1)
    //         .stroke_alignment(StrokeAlignment::Inside)
    //         .fill_color(Rgb555::new(35, 55, 50))
    //         .build();

    //     Rectangle::with_corners(Point::new(12, 12), Point::new(13, 13))
    //         .into_styled(style)
    //         .draw(&mut expected_display)
    //         .unwrap();

    //     display.assert_eq(&expected_display);
    // }
}
