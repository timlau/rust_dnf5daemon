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

// region:    --- Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = Error::TransactionNotResolved("test message".to_string());
        assert_eq!(format!("{}", err), "TransactionNotResolved(\"test message\")");

        let err2 = Error::InvalidTransactionAction("invalid action".to_string());
        assert_eq!(format!("{}", err2), "InvalidTransactionAction(\"invalid action\")");

        let err3 = Error::DnfDaemon("connection failed".to_string());
        assert_eq!(format!("{}", err3), "DnfDaemon(\"connection failed\")");
    }
}

// endregion: --- Unit Tests
