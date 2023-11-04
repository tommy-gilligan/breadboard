#[cfg(feature = "embedded")]
pub mod defmt;
#[cfg(feature = "web")]
pub mod web;

use num_enum::IntoPrimitive;

#[derive(Clone, IntoPrimitive)]
#[repr(u8)]
pub enum XPin {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
}

#[derive(Clone, IntoPrimitive)]
#[repr(u8)]
pub enum YPin {
    Y0,
    Y1,
    Y2,
    Y3,
    Y4,
    Y5,
    Y6,
    Y7,
}

pub fn address<X, Y, const Y_SHIFT: u8>(x: X, y: Y) -> u8
where
    X: Into<u8>,
    Y: Into<u8>,
{
    let x: u8 = x.into();
    let y: u8 = y.into();

    x | (y << Y_SHIFT)
}

pub trait SwitchArray {
    type X: Into<u8>;
    type Y: Into<u8>;

    fn on(&mut self, x: Self::X, y: Self::Y);

    fn off(&mut self, x: Self::X, y: Self::Y);
}
