pub struct Xpt2046<S: embedded_hal::spi::SpiDevice> {
    spi: S,
}

impl<S: embedded_hal::spi::SpiDevice> Xpt2046<S> {
    pub fn new(mut spi_device: S) -> Self {
        let mut rx_buff = [0; 5];
        spi_device
            .transfer(&mut rx_buff, &[0x80, 0, 0, 0, 0])
            .unwrap();
        Self { spi: spi_device }
    }

    pub fn get(&mut self) -> (f32, f32) {
        let mut rx_buff = [0; 5];
        self.spi
            .transfer(&mut rx_buff, &[0x12, 0, 0x1A, 0, 0])
            .unwrap();
        let x = (rx_buff[1] as i32) << 8 | rx_buff[2] as i32;
        let y = (rx_buff[3] as i32) << 8 | rx_buff[4] as i32;

        (
            0.0006100 * x as f32 + 0.0647828 * y as f32 + -13.634,
            0.0890609 * x as f32 + 0.0001381 * y as f32 + -35.73,
        )
    }
}
