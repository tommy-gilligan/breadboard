pub struct Xpt2046<S: embedded_hal::spi::SpiDevice>(S);

impl<S: embedded_hal::spi::SpiDevice> Xpt2046<S> {
    pub fn new(spi_device: S) -> Self {
        Self(spi_device)
    }

    pub fn get(&mut self) -> Result<(u16, u16), <S as embedded_hal::spi::ErrorType>::Error> {
        let mut buff = [0x12, 0, 0x1A, 0, 0];
        self.0.transfer_in_place(&mut buff)?;
        Ok(
            (
                u16::from_be_bytes([buff[1], buff[2]]),
                u16::from_be_bytes([buff[3], buff[4]]),
            )
        )
    }
}

#[must_use] pub fn convert((x, y): (u16, u16)) -> Option<(u16, u16)> {
    if x < 250 || y < 230 || x > 4000 || y > 3900 {
        return None
    }

    Some(
        (
            (x - 250).wrapping_shr(9) * 68,
            (y - 230).wrapping_shr(9) * 46,
        )
    )
}

#[cfg(test)]
mod test {
    use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
    extern crate std;

    #[test]
    fn test_get() {
        let expectations = [
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x00, 0x00, 0x00, 0x00]
            ),
            SpiTransaction::transaction_end(),

            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x10, 0x00, 0x10, 0x00]
            ),
            SpiTransaction::transaction_end(),

            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x00, 0x00, 0x10, 0x00]
            ),
            SpiTransaction::transaction_end(),

            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x10, 0x00, 0x00, 0x00]
            ),
            SpiTransaction::transaction_end(),

            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x07, 0xD0, 0x00, 0x00]
            ),
            SpiTransaction::transaction_end(),

            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x00, 0x00, 0x07, 0xD0]
            ),
            SpiTransaction::transaction_end(),
        ];

        let mut spi = SpiMock::new(&expectations);
        assert_eq!(super::Xpt2046::new(spi.clone()).get(), Ok((0, 0)));
        assert_eq!(super::Xpt2046::new(spi.clone()).get(), Ok((4096, 4096)));
        assert_eq!(super::Xpt2046::new(spi.clone()).get(), Ok((0, 4096)));
        assert_eq!(super::Xpt2046::new(spi.clone()).get(), Ok((4096, 0)));

        assert_eq!(super::Xpt2046::new(spi.clone()).get(), Ok((2000, 0)));
        assert_eq!(super::Xpt2046::new(spi.clone()).get(), Ok((0, 2000)));

        spi.done();
    }

    #[test]
    fn test_convert() {
        assert_eq!(super::convert((250, 230)), Some((0, 0)));
        assert_eq!(super::convert((3920, 3850)), Some((476, 322)));
    }

    #[test]
    fn test_convert_out_of_range() {
        assert_eq!(super::convert((200, 200)), None);
        assert_eq!(super::convert((4000, 4000)), None);
    }
}
