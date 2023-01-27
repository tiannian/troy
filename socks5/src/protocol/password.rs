use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::Result;

pub async fn recv_password(
    stream: &mut (impl AsyncRead + Unpin),
) -> Result<(u8, Vec<u8>, Vec<u8>)> {
    let ver = stream.read_u8().await?;

    let ulen = stream.read_u8().await?;
    let mut username = vec![0u8; ulen as usize];
    stream.read(&mut username).await?;

    let plen = stream.read_u8().await?;
    let mut password = vec![0u8; plen as usize];
    stream.read(&mut password).await?;

    Ok((ver, username, password))
}

pub async fn send_password_result(
    version: u8,
    result: bool,
    stream: &mut (impl AsyncWrite + Unpin),
) -> Result<()> {
    stream.write_u8(version).await?;

    let res = if result { 0 } else { 1 };
    stream.write_u8(res).await?;

    Ok(())
}
