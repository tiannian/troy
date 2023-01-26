use tokio::net::TcpStream;
use tokio_native_tls::{native_tls, TlsConnector, TlsStream};

use crate::Result;

pub struct Tls {
    pub stream: TlsStream<TcpStream>,
}

impl Tls {
    pub async fn connect(domain: &str, remote: &str) -> Result<Self> {
        let socket = TcpStream::connect(remote).await?;
        let c = native_tls::TlsConnector::new()?;
        let c = TlsConnector::from(c);

        let stream = c.connect(domain, socket).await?;

        Ok(Self { stream })
    }
}
