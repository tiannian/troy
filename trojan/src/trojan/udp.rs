use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{Addr, Error, Result};

pub async fn recv_udp(stream: &mut (impl AsyncRead + Unpin)) -> Result<Vec<u8>> {
    let atype = stream.read_u8().await?;
    if atype == 0x01 {
        let mut ip = [0u8; 4];
        stream.read(&mut ip).await?;
    } else if atype == 0x03 {
        let length = stream.read_u16().await?;
        let mut domain = vec![0u8; length as usize];
        stream.read(&mut domain).await?;
    } else if atype == 0x04 {
        let mut ip = [0u8; 16];
        stream.read(&mut ip).await?;
    } else {
        return Err(Error::UnexpectAddressType);
    }

    let _port = stream.read_u16().await?;
    let length = stream.read_u16().await?;
    let _padding = stream.read_u16().await?;

    let mut data = vec![0u8; length as usize];

    stream.read(&mut data).await?;

    Ok(data)
}

pub async fn send_packet(
    dest_addr: Addr,
    dest_port: u16,
    data: &[u8],
    stream: &mut (impl AsyncWrite + Unpin),
) -> Result<()> {
    stream.write_u8(dest_addr.to_u8()).await?;
    stream.write(&dest_addr.to_vec()).await?;
    stream.write_u16(dest_port).await?;
    stream.write_u16(data.len() as u16).await?;
    stream.write(data).await?;

    Ok(())
}
