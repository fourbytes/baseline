use smol_str::SmolStr;
use time::OffsetDateTime;

use crate::quote::TopOfBook;

#[derive(Debug, Clone, Default)]
pub struct OrderBook<O> where O: Order {
    pub bids: Vec<O>,
    pub asks: Vec<O>,
}

pub trait Order {
    type Price;
    fn price(&self) -> Self::Price;
    fn set_price(&mut self, price: Self::Price);

    type Volume;
    fn volume(&self) -> Self::Volume;
    fn set_volume(&mut self, volume: Self::Volume);

    /// Data source or exchange
    fn source(&self) -> Option<SmolStr> {
        None
    }
    #[allow(unused_variables)]
    fn set_source(&mut self, source: Option<SmolStr>) {}

    /// Timestamp when the order was last updated or created.
    fn last_updated(&self) -> Option<OffsetDateTime> {
        None
    }
    #[allow(unused_variables)]
    fn set_last_updated(&mut self, timestamp: Option<OffsetDateTime>) {}
}

/// Represents the side of an order.
#[derive(Copy, Clone, Debug)]
pub enum Side {
    Bid,
    Ask
}

#[derive(Clone, Debug)]
pub enum Operation<O> where O: Order {
    Insert(O),
    Update(O),
    Remove
}

impl<O: Order> OrderBook<O> {
    pub fn update_by_index(&mut self, side: Side, operation: Operation<O>, index: usize) {
        let orders = match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks
        };
        match operation {
            Operation::Insert(order) => orders.insert(index, order),
            Operation::Update(order) => {
                orders[index].set_price(order.price());
                orders[index].set_volume(order.volume());
                orders[index].set_source(order.source());
                orders[index].set_last_updated(order.last_updated());
            },
            Operation::Remove => {
                orders.remove(index);
            }
        };
    }
}

impl<O: Order> TopOfBook for OrderBook<O> {
    type Price = O::Price;
    type Volume = O::Volume;

    fn bid_price(&self) -> Option<Self::Price> {
        self.bids.get(0).map(Order::price)
    }

    fn bid_volume(&self) -> Option<Self::Volume> {
        self.bids.get(0).map(Order::volume)
    }

    fn ask_price(&self) -> Option<Self::Price> {
        self.asks.get(0).map(Order::price)
    }

    fn ask_volume(&self) -> Option<Self::Volume> {
        self.asks.get(0).map(Order::volume)
    }
}
