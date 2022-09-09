use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

use super::MONEY_SHIFT;
use crate::error::Error;

// ASSUMPTION : 64bit signed integer would not be overflowed
// internally, keep all money with integer.
pub struct Account {
    available: AtomicI64,
    held: AtomicI64,
    locked: AtomicBool,
}

impl Account {
    pub fn new() -> Self {
        Account {
            available: AtomicI64::new(0),
            held: AtomicI64::new(0),
            locked: AtomicBool::new(false),
        }
    }

    pub fn deposit(&mut self, amount: i64) -> Result<i64, Error> {
        if self.locked.load(Ordering::Relaxed) == true {
            return Err(Error::AccountIsLocked);
        }

        Ok(self.available.fetch_add(amount, Ordering::Relaxed))
    }
    pub fn withdraw(&mut self, amount: i64) -> Result<i64, Error> {
        if self.locked.load(Ordering::Relaxed) == true {
            return Err(Error::AccountIsLocked);
        }

        if self.available.load(Ordering::Relaxed) < amount {
            return Err(Error::NotEnoughFunds);
        }

        Ok(self.available.fetch_sub(amount, Ordering::Relaxed))
    }
    pub fn dispute(&mut self, amount: i64) -> Result<i64, Error> {
        if self.locked.load(Ordering::Relaxed) == true {
            return Err(Error::AccountIsLocked);
        }

        // ASSUMPTION : dispute amount cannot exceed available funds
        if self.available.load(Ordering::Relaxed) < amount {
            return Err(Error::NotEnoughFunds);
        }

        self.held.fetch_add(amount, Ordering::Relaxed);
        Ok(self.available.fetch_sub(amount, Ordering::Relaxed))
    }
    pub fn resolve(&mut self, amount: i64) -> Result<i64, Error> {
        if self.locked.load(Ordering::Relaxed) == true {
            return Err(Error::AccountIsLocked);
        }

        self.held.fetch_sub(amount, Ordering::Relaxed);
        Ok(self.available.fetch_add(amount, Ordering::Relaxed))
    }
    pub fn chargeback(&mut self, amount: i64) -> Result<i64, Error> {
        if self.locked.load(Ordering::Relaxed) == true {
            return Err(Error::AccountIsLocked);
        }

        self.locked.store(true, Ordering::Relaxed);
        self.held.fetch_sub(amount, Ordering::Relaxed);
        Ok(self.available.load(Ordering::Relaxed))
    }
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.available.load(Ordering::Relaxed) as f64 / MONEY_SHIFT,
            self.held.load(Ordering::Relaxed) as f64 / MONEY_SHIFT,
            (self.available.load(Ordering::Relaxed) + self.held.load(Ordering::Relaxed)) as f64
                / MONEY_SHIFT,
            self.locked.load(Ordering::Relaxed)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_and_withdrawal() {
        let mut account = Account::new();

        account.deposit(1000).unwrap();
        account.withdraw(1000).unwrap();

        assert_eq!(account.available.load(Ordering::Relaxed), 0);
        assert_eq!(account.held.load(Ordering::Relaxed), 0);

        account.deposit(100).unwrap();
        account.deposit(200).unwrap();

        assert_eq!(account.available.load(Ordering::Relaxed), 300);
        assert_eq!(account.held.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn dispute_flow() {
        let mut account = Account::new();

        account.deposit(1000).unwrap();
        account.dispute(500).unwrap();

        assert_eq!(account.available.load(Ordering::Relaxed), 500);
        assert_eq!(account.held.load(Ordering::Relaxed), 500);

        account.resolve(500).unwrap();

        assert_eq!(account.available.load(Ordering::Relaxed), 1000);
        assert_eq!(account.held.load(Ordering::Relaxed), 0);

        account.dispute(500).unwrap();
        account.chargeback(500).unwrap();

        assert_eq!(account.available.load(Ordering::Relaxed), 500);
        assert_eq!(account.held.load(Ordering::Relaxed), 0);
        assert_eq!(account.locked.load(Ordering::Relaxed), true);
    }

    #[test]
    fn lock() {
        let mut account = Account::new();

        account.deposit(10).unwrap();
        account.dispute(5).unwrap();
        account.chargeback(5).unwrap();
        assert!(account.deposit(20).is_err());
    }

    #[test]
    fn not_enough_funds() {
        let mut account = Account::new();
        assert!(account.withdraw(20).is_err());
    }
}
