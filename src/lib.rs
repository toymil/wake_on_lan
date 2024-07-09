use std::{
    net::{IpAddr, Ipv4Addr, UdpSocket},
    str::FromStr,
};

use clap::Parser;
use result_dyn::{msg_boxed, DynSyncError, ResultDyn};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Parser)]
#[command(version)]
pub struct Config {
    #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::UNSPECIFIED))]
    bind_ip: IpAddr,
    #[arg(long, default_value_t = 0)]
    bind_port: u16,

    #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::BROADCAST))]
    ip: IpAddr,
    #[arg(long, default_value_t = 0)]
    port: u16,

    #[arg(
        required = true,
        value_name = "MAC_ADDR",
        value_parser = <MacAddr as FromStr>::from_str,
    )]
    mac_addr_list: Vec<MacAddr>,
}

impl Config {
    pub fn wake(self) -> ResultDyn<()> {
        let socket = UdpSocket::bind((self.bind_ip, self.bind_port))?;
        socket.set_broadcast(true)?;

        socket.connect((self.ip, self.port))?;
        for magic_packet in self
            .mac_addr_list
            .into_iter()
            .map(MacAddr::into_magic_packet)
        {
            socket.send(&magic_packet)?;
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MacAddr([u8; 6]);

impl MacAddr {
    pub fn into_magic_packet(self) -> [u8; 6 + 6 * 16] {
        let mut packet: Vec<u8> = Vec::new();

        packet.extend([u8::MAX; 6]);
        packet.extend([self.0; 16].concat());

        return packet
            .try_into()
            .expect("this should be exactly the right length");
    }
}

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
