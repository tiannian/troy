use std::net::SocketAddr;

use crate::Result;

pub struct UdpTunDevice {}

impl UdpTunDevice {
    pub fn recv_packet(from: SocketAddr, to: SocketAddr, data: &[u8]) -> Result<()> {
        Ok(())
    }
}
