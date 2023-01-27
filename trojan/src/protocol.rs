use std::net::{Ipv4Addr, Ipv6Addr};

pub enum Addr {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    Domain(String),
}

impl Addr {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Ipv4(_) => 1,
            Self::Domain(_) => 3,
            Self::Ipv6(_) => 4,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Ipv4(a) => a.octets().to_vec(),
            Self::Ipv6(a) => a.octets().to_vec(),
            Self::Domain(d) => {
                let len = d.len();
                let mut buff = Vec::with_capacity(1 + len);
                buff.push(len as u8);
                buff.extend_from_slice(d.as_bytes());
                buff
            }
        }
    }
}

pub enum Command {
    Connect,
    UdpAssociate,
}

impl From<Command> for u8 {
    fn from(value: Command) -> Self {
        match value {
            Command::Connect => 1,
            Command::UdpAssociate => 2,
        }
    }
}

pub const CRLF: [u8; 2] = [0x0d, 0x0a];
