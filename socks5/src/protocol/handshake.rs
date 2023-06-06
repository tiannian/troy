use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{Error, Result};

pub enum Method {
    NoAuthentication,
    UsernamePassword,
    NoAcceptable,
}

impl Method {
    pub fn to_u8(&self) -> u8 {
        match self {
            Method::NoAuthentication => 0x00,
            Method::UsernamePassword => 0x02,
            Method::NoAcceptable => 0xff,
        }
    }

    pub fn from_u8(b: u8) -> Result<Self> {
        match b {
            0x00 => Ok(Self::NoAuthentication),
            0x01 => Ok(Self::UsernamePassword),
            0xff => Ok(Self::NoAcceptable),
            _ => Err(Error::UnsupportedMethd(b)),
        }
    }
}

pub async fn recv_handshake(stream: &mut (impl AsyncRead + Unpin)) -> Result<Vec<Method>> {
    let ver = stream.read_u8().await?;

    if ver != 0x05 {
        return Err(Error::NoSocksVersionSupport);
    }

    let methods_len = stream.read_u8().await?;
    let mut methods = vec![0u8; methods_len as usize];
    stream.read_buf(&mut methods).await?;

    let mut res = Vec::with_capacity(methods_len as usize);
    for m in methods {
        res.push(Method::from_u8(m)?)
    }

    Ok(res)
}

pub async fn send_handshake(method: Method, stream: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
    stream.write_u8(0x05).await?;
    stream.write_u8(method.to_u8()).await?;

    Ok(())
}
