use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub trait OHLC {
    type Type;

    fn open(&self) -> Self::Type;
    fn high(&self) -> Self::Type;
    fn low(&self) -> Self::Type;
    fn close(&self) -> Self::Type;
}

pub trait OHLCMath: OHLC {
    fn ohlc4(&self) -> Self::Type;
    fn hlc3(&self) -> Self::Type;
    fn hl2(&self) -> Self::Type;
}

impl OHLCMath for dyn OHLC<Type = Decimal> {
    fn ohlc4(&self) -> Decimal {
        (self.open() + self.high() + self.low() + self.close()) / dec!(4)
    }

    fn hlc3(&self) -> Decimal {
        (self.high() + self.low() + self.close()) / dec!(3)
    }

    fn hl2(&self) -> Decimal {
        (self.high() + self.low()) / dec!(2)
    }
}

impl OHLCMath for dyn OHLC<Type = f64> {
    fn ohlc4(&self) -> f64 {
        (self.open() + self.high() + self.low() + self.close()) / 4.0
    }

    fn hlc3(&self) -> f64 {
        (self.high() + self.low() + self.close()) / 3.0
    }

    fn hl2(&self) -> f64 {
        (self.high() + self.low()) / 2.0
    }
}

pub trait Volume {
    type Type;

    fn volume(&self) -> Self::Type;
}
