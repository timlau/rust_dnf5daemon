use derive_more::{Display, From};
use zbus;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
    TransactionNotResolved(String),
    #[from]
    DBus(zbus::Error),
    DnfDaemon(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}
