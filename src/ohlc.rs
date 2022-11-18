use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub trait OHLC {
    fn open(&self) -> Decimal;
    fn high(&self) -> Decimal;
    fn low(&self) -> Decimal;
    fn close(&self) -> Decimal;
}

impl dyn OHLC {
    pub fn ohlc4(&self) -> Decimal {
        (self.open() + self.high() + self.low() + self.close()) / dec!(4)
    }

    pub fn hlc3(&self) -> Decimal {
        (self.high() + self.low() + self.close()) / dec!(3)
    }

    pub fn hl2(&self) -> Decimal {
        (self.high() + self.low()) / dec!(2)
    }
}
