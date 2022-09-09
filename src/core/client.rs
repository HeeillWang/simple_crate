use std::collections::HashMap;

use super::bank::Account;
use super::query::*;
use super::AccountOwner;
use super::TxId;
use crate::error::Error;

pub struct Client {
    id: u16,
    account: Account,
    tx_map: HashMap<TxId, (i64, bool)>, // (amount, is_under_dispute)
}

impl Client {
    pub fn new(id: u16) -> Self {
        Client {
            id,
            account: Account::new(),
            tx_map: HashMap::new(),
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }
}

impl AccountOwner for Client {
    fn dispatch(&mut self, query: Query) -> Result<i64, Error> {
        match query.query_type.to_lowercase().as_str() {
            QUERY_TYPE_DEPOSIT => {
                if self.tx_map.get(&query.tx).is_some() {
                    Err(Error::DuplicatedTx)
                } else {
                    self.deposit(query.tx, query.amount())
                }
            }
            QUERY_TYPE_WITHDRAWAL => {
                if self.tx_map.get(&query.tx).is_some() {
                    Err(Error::DuplicatedTx)
                } else {
                    self.withdraw(query.tx, query.amount())
                }
            }
            QUERY_TYPE_DISPUTE => self.dispute(query.tx),
            QUERY_TYPE_RESOLVE => self.resolve(query.tx),
            QUERY_TYPE_CHARGEBACK => self.chargeback(query.tx),
            _ => Err(Error::InvalidQueryType(query.query_type.clone())),
        }
    }
}

impl Client {
    fn deposit(&mut self, _tx: TxId, amount: i64) -> Result<i64, Error> {
        self.account.deposit(amount)
    }
    fn withdraw(&mut self, tx: TxId, amount: i64) -> Result<i64, Error> {
        self.tx_map.insert(tx, (amount, false));
        self.account.withdraw(amount)
    }
    fn dispute(&mut self, tx: TxId) -> Result<i64, Error> {
        let (amount, is_dispute) = self.tx_map.get_mut(&tx).ok_or(Error::NoTxFound)?;

        if *is_dispute {
            return Err(Error::AlreadyDisputed);
        }

        *is_dispute = true;
        self.account.dispute(*amount)
    }
    fn resolve(&mut self, tx: TxId) -> Result<i64, Error> {
        let (amount, is_dispute) = self.tx_map.get_mut(&tx).ok_or(Error::NoTxFound)?;

        if !*is_dispute {
            return Err(Error::NotUnderDispute);
        }

        // ASSUMPTION : resolved transaction cannot be disputed again
        // *is_dispute = false;
        self.account.resolve(*amount)
    }
    fn chargeback(&mut self, tx: TxId) -> Result<i64, Error> {
        let (amount, is_dispute) = self.tx_map.get_mut(&tx).ok_or(Error::NoTxFound)?;

        if !*is_dispute {
            return Err(Error::NotUnderDispute);
        }

        // ASSUMPTION : charge-backed transaction cannot be disputed again
        // *is_dispute = false;
        let ret = self.account.chargeback(*amount);
        ret
    }
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.id, self.account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_under_dispute() {
        let mut client = Client::new(1);

        client.deposit(1, 100).unwrap();
        assert!(client.resolve(1).is_err());
    }

    #[test]
    fn dispute() {
        let mut client = Client::new(1);

        client.deposit(1, 100).unwrap();
        client.withdraw(2, 50).unwrap();
        client.dispute(2).unwrap();
        assert!(client.resolve(2).is_ok());
        assert!(client.dispute(2).is_err());
    }
}
