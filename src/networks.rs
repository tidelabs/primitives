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

pub type Addresses = BTreeMap<String, String>;

pub type ChainIds = BTreeMap<String, u32>;

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
            ids.insert("Local".to_string(), 1337);
            ids.insert("Devnet".to_string(), 3);
            ids.insert("Testnet".to_string(), 3);
            ids.insert("Mainnet".to_string(), 1);
            return Some(ids);
        }
        None
    }
    pub fn router(&self) -> Option<Addresses> {
        if self == &Asset::Ethereum {
            return Some(str_map(vec![
                ("Local", "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"),
                ("Testnet", "0x9d2aebec4b3b9ba6e6a57c889cfbcdc9b52f46d4"),
            ]));
        }
        None
    }
    pub fn multisig(&self) -> Option<Addresses> {
        if self == &Asset::Ethereum {
            return Some(str_map(vec![
                ("Local", "0x5fc8d32690cc91d4c39d9d3abcbd16989f875707"),
                ("Testnet", "0x1d649c81f979d6df3c5aa8f0e06a1d5e8c9a7b91"),
            ]));
        }
        None
    }
    pub fn address(&self) -> Option<Addresses> {
        if self == &Asset::Tether {
            return Some(str_map(vec![
                ("Local", "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0"),
                ("Testnet", "0x91e6198f5cf80a6c47212e440e72bf4e052ce148"),
            ]));
        } else if self == &Asset::USDCoin {
            return Some(str_map(vec![(
                "Testnet",
                "0x698638ba2c96e49cb5387354925f95bceef8e9f3",
            )]));
        }
        None
    }
}

fn str_map(inp: Vec<(&str, &str)>) -> BTreeMap<String, String> {
    let mut r = BTreeMap::new();
    for (one, two) in inp {
        r.insert(one.to_string(), two.to_string());
    }
    r
}
