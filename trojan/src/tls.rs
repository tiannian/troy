use std::net::SocketAddr;

use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{lookup_host, TcpSocket},
};
use tokio_native_tls::{native_tls, TlsConnector};

use crate::{config::Config, Error, Result};

pub async fn connect(config: &Config) -> Result<impl AsyncRead + AsyncWrite + Unpin> {
    let tcp = {
        let host = format!("{}:{}", config.remote_addr, config.remote_port);

        let mut remotes = lookup_host(host).await?;

        let remote = remotes.next().ok_or(Error::DomainResolveFailed)?;

        let socket = match &remote {
            SocketAddr::V4(_) => TcpSocket::new_v4()?,
            SocketAddr::V6(_) => TcpSocket::new_v6()?,
        };

        if let Some(b) = &config.tcp.reuse_port {
            socket.set_reuseaddr(*b)?;
        }

        #[cfg(any(target_os = "linux", target_os = "fuchsia", target_os = "android"))]
        if let Some(b) = &config.tcp.bind_device {
            socket.bind_device(Some(b.as_bytes()))?;
        }

        let stream = socket.connect(remote).await?;

        if let Some(nodelay) = config.tcp.no_delay {
            stream.set_nodelay(nodelay)?;
        }

        stream
    };

    let connector = native_tls::TlsConnector::new()?;
    let connector = TlsConnector::from(connector);

    let stream = connector.connect(&config.remote_addr, tcp).await?;

    Ok(stream)
}
