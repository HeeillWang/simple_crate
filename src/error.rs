#[derive(Debug)]
pub enum Error {
    NoTxFound,
    CSVConvertFail,
    InvalidQueryType(String),
    InaccurateQueryType(String),
    DuplicatedTx,
    InputArgIsNotProvided,
    AccountIsLocked,
    NotEnoughFunds,
    NotUnderDispute,
    AlreadyDisputed,
}
