use defmt::println;

pub struct CH446Q;

impl super::SwitchArray for CH446Q {
    type X = super::XPin;
    type Y = super::YPin;

    fn on(&mut self, x: Self::X, y: Self::Y) {
        println!(
            "On X{} Y{} address {}",
            u8::from(x.clone()),
            u8::from(y.clone()),
            super::address::<Self::X, Self::Y, 4>(x, y)
        );
    }

    fn off(&mut self, x: Self::X, y: Self::Y) {
        println!(
            "Off X{} Y{} address {}",
            u8::from(x.clone()),
            u8::from(y.clone()),
            super::address::<Self::X, Self::Y, 4>(x, y)
        );
    }
}
