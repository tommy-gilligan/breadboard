pub struct Xpt2046<S: embedded_hal::spi::SpiDevice>(S);

impl<S: embedded_hal::spi::SpiDevice> Xpt2046<S> {
    pub fn new(spi_device: S) -> Self {
        Self(spi_device)
    }

    pub fn get(&mut self) -> Result<(u16, u16), <S as embedded_hal::spi::ErrorType>::Error> {
        let mut buff = [0x12, 0, 0x1A, 0, 0];
        self.0.transfer_in_place(&mut buff)?;
        Ok((
            u16::from_be_bytes([buff[1], buff[2]]),
            u16::from_be_bytes([buff[3], buff[4]]),
        ))
    }
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
                std::vec![0x00, 0x00, 0x00, 0x00, 0x00],
            ),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x10, 0x00, 0x10, 0x00],
            ),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x00, 0x00, 0x10, 0x00],
            ),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x10, 0x00, 0x00, 0x00],
            ),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x07, 0xD0, 0x00, 0x00],
            ),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(
                std::vec![0x12, 0x00, 0x1A, 0x00, 0x00],
                std::vec![0x00, 0x00, 0x00, 0x07, 0xD0],
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
}
