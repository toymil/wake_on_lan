use std::str::FromStr;

use clap::Parser;
use result_dyn::{msg_boxed, DynSyncError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Parser)]
#[command(version)]
pub struct Cli {
    #[arg(required = true, value_parser = <MacAddr as FromStr>::from_str)]
    mac_addresses: Vec<MacAddr>,
}

impl Cli {
    pub fn run(self) {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct MacAddr([u8; 6]);

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
