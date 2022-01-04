use crate::assets::Asset;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::str::FromStr;
use strum_macros::{Display, EnumIter};

#[derive(
    Clone, PartialEq, Eq, Ord, PartialOrd, EnumIter, Debug, Serialize, Deserialize, Hash, Display,
)]
pub enum Network {
    Local,
    Devnet,
    Testnet,
    Mainnet,
}

pub type Addresses = BTreeMap<&'static str, &'static str>;

pub type ChainIds = BTreeMap<&'static str, u32>;

impl FromStr for Network {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Local" {
            return Ok(Network::Local);
        } else if s == "Devnet" {
            return Ok(Network::Devnet);
        } else if s == "Testnet" {
            return Ok(Network::Testnet);
        } else if s == "Mainnet" {
            return Ok(Network::Mainnet);
        }
        Err("invalid network string".to_string())
    }
}

impl Asset {
    pub fn chain_id(&self) -> Option<ChainIds> {
        if self == &Asset::Ethereum {
            let mut ids = BTreeMap::new();
            ids.insert("Local", 1337);
            ids.insert("Devnet", 3);
            ids.insert("Testnet", 3);
            ids.insert("Mainnet", 1);
            return Some(ids);
        }
        None
    }
    pub fn router(&self) -> Option<Addresses> {
        if self == &Asset::Ethereum {
            let mut addys = BTreeMap::new();
            addys.insert("Local", "0xdbd4910f54a3751f964cb3bad99374134b2e34e7");
            addys.insert("Testnet", "0x9d2aebec4b3b9ba6e6a57c889cfbcdc9b52f46d4");
            return Some(addys);
        }
        None
    }
    pub fn multisig(&self) -> Option<Addresses> {
        if self == &Asset::Ethereum {
            let mut addys = BTreeMap::new();
            addys.insert("Local", "0xeed55a8e858d98371330d990358360ee36eeee6f");
            addys.insert("Testnet", "0x1d649c81f979d6df3c5aa8f0e06a1d5e8c9a7b91");
            return Some(addys);
        }
        None
    }
    pub fn address(&self) -> Option<Addresses> {
        if self == &Asset::Tether {
            let mut addys = BTreeMap::new();
            addys.insert("Local", "0x348484e4a9a95dbd667398fe4f4fa6d4aaae4e18");
            addys.insert("Testnet", "0x91e6198f5cf80a6c47212e440e72bf4e052ce148");
            return Some(addys);
        } else if self == &Asset::USDCoin {
            let mut addys = BTreeMap::new();
            addys.insert("Testnet", "0x698638ba2c96e49cb5387354925f95bceef8e9f3");
            return Some(addys);
        }
        None
    }
}
