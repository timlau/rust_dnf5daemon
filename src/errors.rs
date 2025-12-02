use derive_more::From;

/// Result type used throughout the dnf5daemon library
pub type Result<T> = std::result::Result<T, Error>;

// region:    --- Error
/// Enum representing possible errors in dnf5daemon operations
#[derive(Debug, From)]
pub enum Error {
    /// Error indicating that a transaction has not been resolved
    TransactionNotResolved(String),
    /// Error indicating an invalid transaction action
    InvalidTransactionAction(String),
    /// DBus related error
    #[from]
    DBus(zbus::Error),
    /// Error indicating failure to connect to DnfDaemon
    DnfDaemon(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error
