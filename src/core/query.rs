use super::{TxId, UserId, MONEY_SHIFT};
use serde::Deserialize;

pub const QUERY_TYPE_DEPOSIT: &str = "deposit";
pub const QUERY_TYPE_WITHDRAWAL: &str = "withdrawal";
pub const QUERY_TYPE_DISPUTE: &str = "dispute";
pub const QUERY_TYPE_RESOLVE: &str = "resolve";
pub const QUERY_TYPE_CHARGEBACK: &str = "chargeback";

// csv has issue to deserialize enum
// https://github.com/BurntSushi/rust-csv/issues/211
// #[derive(Debug, Deserialize)]
// #[serde(tag = "type")]
// pub enum Query {
//     Deposit { user: UserId, tx: u16, amount: u64 },
//     Withdraw { user: UserId, tx: u16, amount: u64 },
//     Dispute { user: UserId, tx: u16 },
//     Resolve { user: UserId, tx: u16 },
//     ChargeBack { user: UserId, tx: u16 },
// }

#[derive(Debug, Deserialize)]
pub struct Query {
    #[serde(alias = "type")] // 'type' is a keyword in rust
    pub query_type: String,
    #[serde(alias = "client")]
    user_id: UserId,
    pub tx: TxId,
    amount: f64,
}

impl Query {
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn amount(&self) -> i64 {
        (self.amount * MONEY_SHIFT) as i64
    }
}
