use std::collections::HashMap;

use super::orderbook::{Order, Orderbook};
use rust_decimal::prelude::*;
// BTCUSD
// BTC = Base
// USD = Quote
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> Self {
        Self { base, quote }
    }

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("Opening new orderbook for market {:?}", pair.to_string())
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: Decimal,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(order_book) => {
                order_book.add_order(price, order);
                println!("placed limit order at price {}", pair.to_string());
                Ok(())
            }
            None => Err(format!(
                "the orderbook for the given tradign pair ({}) does not exist",
                pair.to_string()
            )),
        }
    }
}
