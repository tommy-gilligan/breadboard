use embedded_graphics::{
    geometry::{OriginDimensions, Size},
    pixelcolor::Rgb888,
    prelude::{Point, Primitive},
    primitives::{Line, PrimitiveStyleBuilder, Rectangle},
    Drawable,
};

use embedded_graphics_core::draw_target::DrawTarget;

use core::fmt::Debug;

use crate::model;

const TRENCH: i32 = 38;
const VERTICAL_OFFSET: i32 = 51;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Region {
    Top,
    Bottom,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HitTestResult {
    HitColumn((Region, usize)),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectionEventType {
    Start,
    Update,
    End,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelectionEvent {
    pub region: Region,
    pub column: usize,
    pub r#type: SelectionEventType,
}

#[derive(Clone, Debug, Copy)]
pub struct Row {
    pub columns: [Rgb888; 30],
}

impl Default for Row {
    fn default() -> Self {
        Self {
            columns: [Rgb888::new(0, 0, 0); 30],
        }
    }
}

pub struct Breadboard {
    pub rows: [Row; 10],
    pub point_size: (i32, i32),
    pub source_column: Option<(Region, usize)>,
    pub destination_column: Option<(Region, usize)>,
}

impl Breadboard {
    pub fn new(point_size: (i32, i32)) -> Self {
        let row = Row::default();
        Self {
            source_column: None,
            destination_column: None,
            point_size,
            rows: [row; 10],
        }
    }

    pub fn draw_selection_highlight<DT, E>(&mut self, display: &mut DT)
    where
        E: Debug,
        DT: DrawTarget<Color = Rgb888, Error = E> + OriginDimensions,
    {
        let fill = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::new(255, 0, 0))
            .build();

        if let Some((_region, column)) = &self.source_column {
            let destination_index = if let Some((_region, end_column)) = &self.destination_column {
                end_column
            } else {
                column
            };

            let left = (*column.min(destination_index)) as i32;
            let right = (*column.max(destination_index)) as i32;

            Rectangle::with_corners(
                Point::new(left * self.point_size.0, 0),
                Point::new((right + 1) * self.point_size.0, self.point_size.1),
            )
            .into_styled(fill)
            .draw(display)
            .unwrap();
        }
    }

    // TODO: support bottom
    fn draw_connections<DT, E>(&mut self, display: &mut DT, model: &model::Connections)
    where
        E: Debug,
        DT: DrawTarget<Color = Rgb888, Error = E> + OriginDimensions,
    {
        let mut packed: heapless::Vec<heapless::Vec<model::Connection, 32>, 8> =
            heapless::Vec::new();

        // pack
        let mut model = model.clone();
        while model.iter().count() > 0 {
            // pop
            let popped = *model.iter().next().unwrap();
            model.remove(popped);

            let mut packing: heapless::Vec<model::Connection, 32> = heapless::Vec::new();
            packing.push(popped).unwrap();

            while model.iter().count() > 0 {
                if let Some(popped) = model
                    .clone()
                    .iter()
                    .find(|a| packing.iter().all(|b| a.non_overlapping(b)))
                {
                    model.remove(*popped);
                    packing.push(*popped).unwrap();
                } else {
                    break;
                }
            }
            // packing.sort();
            packed.push(packing).unwrap();
        }

        let color_pairs = [
            [Rgb888::new(255, 255, 0), Rgb888::new(255, 165, 0)],
            [Rgb888::new(255, 0, 0), Rgb888::new(0, 255, 0)],
            [Rgb888::new(0, 0, 255), Rgb888::new(255, 0, 255)],
        ];
        for ((index, row_connections), color_pair) in packed.iter().enumerate().zip(color_pairs) {
            for (row_connection, color) in row_connections.iter().zip(color_pair.iter().cycle()) {
                let fill = PrimitiveStyleBuilder::new().fill_color(*color).build();
                let stroke = PrimitiveStyleBuilder::new()
                    .stroke_color(*color)
                    .stroke_width(1)
                    .build();

                if let model::Connection::Top(start, end) = row_connection {
                    Rectangle::with_corners(
                        Point::new(
                            *start as i32 * self.point_size.0,
                            VERTICAL_OFFSET + index as i32 * self.point_size.1,
                        ),
                        Point::new(
                            *end as i32 * self.point_size.0,
                            VERTICAL_OFFSET + (index as i32 + 1) * self.point_size.1,
                        ),
                    )
                    .into_styled(fill)
                    .draw(display)
                    .unwrap();

                    Line::new(
                        Point::new(*start as i32 * self.point_size.0, 0),
                        Point::new(
                            *start as i32 * self.point_size.0,
                            VERTICAL_OFFSET + (index as i32 + 1) * self.point_size.1,
                        ),
                    )
                    .into_styled(stroke)
                    .draw(display)
                    .unwrap();

                    Line::new(
                        Point::new(*end as i32 * self.point_size.0, 0),
                        Point::new(
                            *end as i32 * self.point_size.0,
                            VERTICAL_OFFSET + (index as i32 + 1) * self.point_size.1,
                        ),
                    )
                    .into_styled(stroke)
                    .draw(display)
                    .unwrap();
                } else if let model::Connection::Bottom(start, end) = row_connection {
                    Rectangle::with_corners(
                        Point::new(
                            *start as i32 * self.point_size.0,
                            269 - index as i32 * self.point_size.1,
                        ),
                        Point::new(
                            *end as i32 * self.point_size.0,
                            269 - (index as i32 + 1) * self.point_size.1,
                        ),
                    )
                    .into_styled(fill)
                    .draw(display)
                    .unwrap();

                    Line::new(
                        Point::new(*start as i32 * self.point_size.0, 320),
                        Point::new(
                            *start as i32 * self.point_size.0,
                            269 - (index as i32 + 1) * self.point_size.1,
                        ),
                    )
                    .into_styled(stroke)
                    .draw(display)
                    .unwrap();

                    Line::new(
                        Point::new(*end as i32 * self.point_size.0, 0),
                        Point::new(
                            *end as i32 * self.point_size.0,
                            269 - (index as i32 + 1) * self.point_size.1,
                        ),
                    )
                    .into_styled(stroke)
                    .draw(display)
                    .unwrap();
                }
            }
        }
    }

    fn draw_background<DT, E>(&mut self, display: &mut DT)
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
                Point::new(index * self.point_size.0, 0),
                Size::new(
                    self.point_size.0.try_into().unwrap(),
                    VERTICAL_OFFSET as u32,
                ),
            )
            .into_styled(fill)
            .draw(display)
            .unwrap();
            Rectangle::new(
                Point::new(
                    index * self.point_size.0,
                    10 * self.point_size.1 + TRENCH + VERTICAL_OFFSET,
                ),
                Size::new(
                    self.point_size.0.try_into().unwrap(),
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
                Point::new(index * self.point_size.0, VERTICAL_OFFSET),
                Size::new(
                    self.point_size.0.try_into().unwrap(),
                    5 * self.point_size.1 as u32,
                ),
            )
            .into_styled(fill)
            .draw(display)
            .unwrap();

            Rectangle::new(
                Point::new(
                    index * self.point_size.0,
                    5 * self.point_size.1 + TRENCH + VERTICAL_OFFSET,
                ),
                Size::new(
                    self.point_size.0.try_into().unwrap(),
                    5 * self.point_size.1 as u32,
                ),
            )
            .into_styled(fill)
            .draw(display)
            .unwrap();
        }
    }

    pub fn draw<DT, E>(&mut self, display: &mut DT, model: &model::Connections)
    where
        E: Debug,
        DT: DrawTarget<Color = Rgb888, Error = E> + OriginDimensions,
    {
        self.draw_background(display);
        self.draw_connections(display, model);
        self.draw_selection_highlight(display);
    }

    pub fn update_selection(&mut self, selection_event: SelectionEvent) -> Option<(usize, usize)> {
        match selection_event {
            SelectionEvent {
                region,
                column,
                r#type: SelectionEventType::Start,
            } => {
                self.destination_column = None;
                self.source_column = Some((region, column));
                None
            }
            SelectionEvent {
                region,
                column,
                r#type: SelectionEventType::Update,
            } => {
                self.destination_column = Some((region, column));
                None
            }
            SelectionEvent {
                region,
                column,
                r#type: SelectionEventType::End,
            } => {
                self.destination_column = Some((region, column));

                let start_index = self.source_column.as_ref().unwrap().1;
                let destination_index = self.destination_column.as_ref().unwrap().1;

                Some((
                    start_index.min(destination_index),
                    start_index.max(destination_index),
                ))
            }
        }
    }

    pub fn hit_test(&self, x: i32, y: i32) -> HitTestResult {
        let region = if y > 160 { Region::Bottom } else { Region::Top };

        HitTestResult::HitColumn((region, x as usize / self.point_size.0 as usize))
    }
}
