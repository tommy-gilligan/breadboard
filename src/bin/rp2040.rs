#![no_std]
#![no_main]
#[cfg(not(target_os = "none"))]
mod other {
    extern crate std;
    use std::println;
    #[no_mangle]
    pub extern "C" fn main() {
        println!("unsupported target");
    }
}

#[cfg(target_os = "none")]
mod embedded {
    #[link_section = ".boot2"]
    #[used]
    pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;
    use application::Controller;

    use core::cell::RefCell;
    use display_interface_spi::SPIInterface;
    use embedded_graphics::prelude::Point;
    use embedded_graphics::prelude::Primitive;
    use embedded_graphics::primitives::Circle;
    use embedded_graphics::primitives::PrimitiveStyle;
    use embedded_graphics::Drawable;

    use defmt::export::display;
    use embedded_graphics::draw_target::DrawTarget;
    use embedded_graphics::pixelcolor::Rgb666;
    use embedded_graphics::pixelcolor::Rgb888;
    use embedded_graphics::prelude::RgbColor;
    use mipidsi::{Builder, ColorOrder, Orientation};

    use defmt_rtt as _;
    use embedded_hal::delay::DelayUs;
    use embedded_hal::digital::OutputPin;
    use embedded_hal::spi::{Mode, Phase, Polarity};
    use embedded_hal_bus::spi::RefCellDevice;
    use fugit::RateExtU32;
    use hal::{
        clocks::{init_clocks_and_plls, Clock},
        gpio::Pins,
        pac,
        sio::Sio,
        watchdog::Watchdog,
        Timer,
    };
    use panic_probe as _;
    use rp2040_hal as hal;

    struct Delay<'a>(&'a mut Timer);

    impl embedded_hal::delay::DelayUs for Delay<'_> {
        fn delay_us(&mut self, d: u32) {
            self.0.delay_us(d);
        }
    }

    #[hal::entry]
    fn main() -> ! {
        let mut pac = pac::Peripherals::take().unwrap();
        let _core = pac::CorePeripherals::take().unwrap();
        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio = Sio::new(pac.SIO);
        let external_xtal_freq_hz = 12_000_000u32;
        let clocks = init_clocks_and_plls(
            external_xtal_freq_hz,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let pins = Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
        let mut delay = Delay(&mut timer);
        let mut vcc = pins.gpio5.into_push_pull_output();
        vcc.set_high().unwrap();
        delay.delay_ms(500);

        let clk_0 = pins.gpio10.into_function::<hal::gpio::FunctionSpi>();
        let mosi_0 = pins.gpio11.into_function::<hal::gpio::FunctionSpi>();
        let miso_0 = pins.gpio8.into_function::<hal::gpio::FunctionSpi>();
        let lcd_cs = pins.gpio9.into_push_pull_output();
        let lcd_rst = pins.gpio13.into_push_pull_output();
        let lcd_dc = pins.gpio12.into_push_pull_output();
        let spi_0 = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI1, (mosi_0, miso_0, clk_0));
        let spi_0 = spi_0.init(
            &mut pac.RESETS,
            clocks.peripheral_clock.freq(),
            50u32.MHz(),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
        );
        let spi_0_cell = RefCell::new(spi_0);
        let lcd_spi_device = RefCellDevice::new_no_delay(&spi_0_cell, lcd_cs);

        let clk_1 = pins.gpio2.into_function::<hal::gpio::FunctionSpi>();
        let mosi_1 = pins.gpio3.into_function::<hal::gpio::FunctionSpi>();
        let miso_1 = pins.gpio4.into_function::<hal::gpio::FunctionSpi>();
        let touch_cs = pins.gpio7.into_push_pull_output();
        let spi_1 = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI0, (mosi_1, miso_1, clk_1));
        let spi_1 = spi_1.init(
            &mut pac.RESETS,
            clocks.peripheral_clock.freq(),
            1u32.MHz(),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
        );
        let spi_1_cell = RefCell::new(spi_1);
        let touch_spi_device = RefCellDevice::new_no_delay(&spi_1_cell, touch_cs);

        let mut screen = Builder::ili9486_rgb666(SPIInterface::new(lcd_spi_device, lcd_dc))
            .with_color_order(ColorOrder::Bgr)
            .with_orientation(Orientation::LandscapeInverted(false))
            .init(&mut delay, Some(lcd_rst))
            .unwrap();

        let mut touchscreen = touchscreen::xpt2046_screen::Xpt2046Screen::new(
            screen,
            touch_spi_device,
            touchscreen::xpt2046_screen::default_calibration,
        );
        let mut controller = Controller::new();
        loop {
            timer.delay_ms(10);
            controller.tick(&mut touchscreen);
        }
    }
}
