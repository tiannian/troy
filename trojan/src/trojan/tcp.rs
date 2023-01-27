use tokio::io::{copy_bidirectional, AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::{build_handshake, Addr, Command, Result};

pub async fn forward_tcp(
    hashed_password: &str,
    dest_addr: Addr,
    dest_port: u16,
    from: &mut (impl AsyncRead + AsyncWrite + Unpin),
    to: &mut (impl AsyncRead + AsyncWrite + Unpin),
) -> Result<()> {
    let buff = build_handshake(Command::Connect, hashed_password, dest_addr, dest_port);
    to.write(&buff).await?;
    copy_bidirectional(from, to).await?;

    Ok(())
}
