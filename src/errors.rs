use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    TransactionNotResolved(String),
    InvalidTransactionAction(String),
    #[from]
    DBus(zbus::Error),
    DnfDaemon(String),
}

// Note: Implement Display as debug, for Web and app error, as anyway those errors will need to be streamed as JSON probably
//       to be rendered for the end user.
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}
