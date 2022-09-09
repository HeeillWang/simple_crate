pub mod bank;
pub mod client;
pub mod query;

use crate::error::Error;

pub type UserId = u16;
pub type TxId = u32;
pub const MONEY_SHIFT: f64 = 10000.0;

pub trait AccountOwner {
    fn dispatch(&mut self, query: query::Query) -> Result<i64, Error>;
}
