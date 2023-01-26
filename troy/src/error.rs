use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    TlsNativeError(#[from] tokio_native_tls::native_tls::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error("Unexpect Address Type")]
    UnexpectAddressType,
}

pub type Result<T> = std::result::Result<T, Error>;
