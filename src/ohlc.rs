use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub trait OHLC {
    type Type;

    fn open(&self) -> Self::Type;
    fn high(&self) -> Self::Type;
    fn low(&self) -> Self::Type;
    fn close(&self) -> Self::Type;
}

impl dyn OHLC<Type = Decimal> {
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

pub trait Volume {
    type Type;

    fn volume(&self) -> Self::Type;
}
