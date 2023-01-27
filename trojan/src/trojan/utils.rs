use bytes::BufMut;

use crate::{Addr, Command, CRLF};

pub fn build_handshake(
    command: Command,
    hashed_password: &str,
    dest_addr: Addr,
    dest_port: u16,
) -> Vec<u8> {
    let mut buff = Vec::with_capacity(100);

    buff.put_slice(hashed_password.as_bytes());
    buff.put_slice(&CRLF);
    buff.put_u8(command.into());
    buff.put_u8(dest_addr.to_u8());
    buff.put_slice(&dest_addr.to_vec());
    buff.put_u16(dest_port);
    buff.put_slice(&CRLF);

    buff
}
