use std::ops::{Add, Div};

/// Includes methods to get the top of book bid & ask.
pub trait TopOfBook {
    type Price;
    type Volume;

    fn bid_price(&self) -> Option<Self::Price>;
    fn bid_volume(&self) -> Option<Self::Volume>;

    fn ask_price(&self) -> Option<Self::Price>;
    fn ask_volume(&self) -> Option<Self::Volume>;

    /// Calculate the mid price from the bid and ask.
    fn mid_price(&self) -> Option<<<Self::Price as Add>::Output as Div<Self::Price>>::Output> where Self::Price: Add + From<u8>, <Self::Price as Add>::Output: Div<Self::Price> {
        if let Some(ask_price) = self.ask_price() {
            self.bid_price().map(|bid_price| (ask_price + bid_price) / Self::Price::from(2u8))
        } else {
            None
        }
    }
}

pub trait LastPrice {
    type Price;

    fn last_price(&self) -> Option<Self::Price>;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    use super::TopOfBook;

    pub struct TopOfBookData {
        bid: Decimal,
        ask: Decimal,
    }

    impl TopOfBook for TopOfBookData {
        type Price = Decimal;

        type Volume = Decimal;

        fn bid_price(&self) -> Option<Self::Price> {
            Some(self.bid)
        }

        fn bid_volume(&self) -> Option<Self::Volume> {
            todo!()
        }

        fn ask_price(&self) -> Option<Self::Price> {
            Some(self.ask)
        }

        fn ask_volume(&self) -> Option<Self::Volume> {
            todo!()
        }
    }

    #[test]
    fn test_mid_price() {
        let data = TopOfBookData {
            bid: 100u8.into(),
            ask: 99u8.into(),
        };
        assert_eq!(data.mid_price(), Decimal::from_str("99.5").ok());
    }
}
