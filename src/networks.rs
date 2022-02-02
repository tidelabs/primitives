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
                ("Devnet", "0xae8a6463bf8449e6b5ee8277924cd6132b809be4"),
                ("Testnet", "0xaa57cd19ae5ed73ea4be754051eb5933d1efd7e0"),
            ]));
        }
        None
    }
    pub fn multisig(&self) -> Option<Addresses> {
        if self == &Asset::Ethereum {
            return Some(str_map(vec![
                ("Local", "0x5fc8d32690cc91d4c39d9d3abcbd16989f875707"),
                ("Devnet", "0x8e0f4a76469096ad322509d4984ee98b10e18ac5"),
                ("Testnet", "0x797939ff57165a46bd0bfa8587f9cb70033b7fb5"),
            ]));
        }
        None
    }

    pub fn address(&self) -> Option<Addresses> {
        if self == &Asset::Tether {
            return Some(str_map(vec![
                ("Local", "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0"),
                ("Devnet", "0xb604ee489aa63aef787a652c606db750b4793e65"),
                ("Testnet", "0xdd60d69de8e211dcaa264142a10e534a68d4ef9d"),
            ]));
        } else if self == &Asset::USDCoin {
            return Some(str_map(vec![
                ("Devnet", "0xf4197f30c8268c933ea57f85c1206e348b54c467"),
                ("Testnet", "0x4170e38d4830f228e3c6e019ad92a29c319c56c2"),
            ]));
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
