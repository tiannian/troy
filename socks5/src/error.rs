use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error("Only support socks5")]
    NoSocksVersionSupport,

    #[error("No support method {0}")]
    UnsupportedMethd(u8),
}

pub type Result<T> = std::result::Result<T, Error>;
