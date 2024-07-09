use std::{
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    str::FromStr,
};

use clap::Parser;
use result_dyn::{msg_boxed, DynSyncError, ResultDyn};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Parser)]
#[command(version)]
pub struct Cli {
    #[arg(
        required = true,
        value_name = "MAC_ADDR",
        value_parser = <MacAddr as FromStr>::from_str,
    )]
    mac_addr_list: Vec<MacAddr>,

    #[arg(long, default_value_t = Ipv4Addr::BROADCAST)]
    ip: Ipv4Addr,

    #[arg(long, default_value_t = 0)]
    port: u16,
}

impl Cli {
    pub fn run(self) -> ResultDyn<()> {
        let socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0))?;
        socket.set_broadcast(true)?;
        socket.connect(SocketAddrV4::new(self.ip, self.port))?;

        for mac_addr in self.mac_addr_list {
            socket.send(&construct_magic_packet(mac_addr))?;
        }

        return Ok(());
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn construct_magic_packet(mac_addr: MacAddr) -> [u8; 6 + 6 * 16] {
    let mut packet: Vec<u8> = Vec::new();
    packet.extend([u8::MAX; 6]);
    packet.extend([mac_addr.0; 16].concat());
    return packet
        .try_into()
        .expect("this should be exactly the right length");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct MacAddr([u8; 6]);

impl FromStr for MacAddr {
    type Err = DynSyncError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .split(':')
            .map(|sub| u8::from_str_radix(sub, 16))
            .collect::<Result<Vec<_>, _>>()?;
        let arr = <[_; 6]>::try_from(bytes).map_err(|vec| {
            msg_boxed!(
                "wrong byte length for mac address, expected 6, got {}",
                vec.len()
            )
        })?;
        return Ok(Self(arr));
    }
}
