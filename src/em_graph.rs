use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
};
use num_traits::float::Float;

use crate::device::Flushable;

#[derive(Debug)]
pub struct State {
    pub frame_count: i32,
    pub point: Point,
}
impl State {
    pub fn new() -> State {
        Self {
            frame_count: 0,
            point: Point::new(0, 0),
        }
    }
}

pub fn setup<D>(display: &mut D, state: &mut State) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor> + Flushable,
{
    state.point.y = 32;
    state.point.x = 64;

    //let style = PrimitiveStyleBuilder::new()
    //    .stroke_width(1)
    //    .stroke_color(BinaryColor::On)
    //    .build();

    //// screen outline
    //// default display size is 128x64 if you don't pass a _DisplaySize_
    //// enum to the _Builder_ struct
    //Rectangle::new(Point::new(0, 0), Size::new(127, 63))
    //    .into_styled(style)
    //    .draw(display)?;
    //
    //// triangle
    //Triangle::new(
    //    Point::new(16, 16 + yoffset),
    //    Point::new(16 + 16, 16 + yoffset),
    //    Point::new(16 + 8, yoffset),
    //)
    //.into_styled(style)
    //.draw(display)?;
    //
    //// square
    //Rectangle::new(Point::new(52, yoffset), Size::new_equal(16))
    //    .into_styled(style)
    //    .draw(display)?;
    //
    //// circle
    //Circle::new(Point::new(88, yoffset), 16)
    //    .into_styled(style)
    //    .draw(display)?;

    display.flush_display();
    Ok(())
}

pub fn update(state: &mut State) {
    state.point.x = 64 - 7 + (Float::sin(state.frame_count as f32 / 10.0) * 20.) as i32;
    state.point.y = 32 - 7 + (Float::cos(state.frame_count as f32 / 10.0) * 20.) as i32;
}

pub fn draw<D>(display: &mut D, state: &mut State) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor> + Flushable,
{
    //display.fill_contiguous(area, colors);

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    //outline screen
    //Rectangle::new(Point::new(0, 0), Size::new(128, 64))
    //    .into_styled(style)
    //    .draw(display)?;

    // compare circles
    //(1..16).for_each(|d| {
    //    let _ = Circle::new(
    //        Point::new((d * 16) % 128, 16 * (1 + d / 8)),
    //        d.try_into().unwrap(),
    //    )
    //    .into_styled(style)
    //    .draw(display);
    //});
    Circle::new(state.point, 13)
        .into_styled(style)
        .draw(display)?;

    Ok(())
}
