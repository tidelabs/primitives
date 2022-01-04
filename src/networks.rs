use crate::assets::Asset;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "std", derive(EnumIter, Debug, Serialize, Deserialize, Hash))]
pub enum Network {
    Devnet,
    Testnet,
    Mainnet,
}

pub struct Addresses {
    pub devnet: String,
    pub testnet: String,
    pub mainnet: String,
}

pub struct ChainIds {
    pub devnet: u32,
    pub testnet: u32,
    pub mainnet: u32,
}

impl FromStr for Network {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Devnet" {
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
            return Some(ChainIds {
                devnet: 1337,
                testnet: 3,
                mainnet: 1,
            });
        }
        None
    }
    pub fn router(&self) -> Option<Addresses> {
        if self == &Asset::Ethereum {
            return Some(Addresses {
                devnet: "0xdbd4910f54a3751f964cb3bad99374134b2e34e7".to_string(),
                testnet: "0x9d2aebec4b3b9ba6e6a57c889cfbcdc9b52f46d4".to_string(),
                mainnet: "".to_string(),
            });
        }
        None
    }
    pub fn multisig(&self) -> Option<Addresses> {
        if self == &Asset::Ethereum {
            return Some(Addresses {
                devnet: "0xeed55a8e858d98371330d990358360ee36eeee6f".to_string(),
                testnet: "0x1d649c81f979d6df3c5aa8f0e06a1d5e8c9a7b91".to_string(),
                mainnet: "".to_string(),
            });
        }
        None
    }
    pub fn address(&self) -> Option<Addresses> {
        match self {
            Asset::Tide => None,
            Asset::Bitcoin => None,
            Asset::Ethereum => None,
            Asset::Tether => Some(Addresses {
                devnet: "0x348484e4a9a95dbd667398fe4f4fa6d4aaae4e18".to_string(),
                testnet: "0x91e6198f5cf80a6c47212e440e72bf4e052ce148".to_string(),
                mainnet: "".to_string(),
            }),
            Asset::USDCoin => Some(Addresses {
                devnet: "".to_string(),
                testnet: "0x698638ba2c96e49cb5387354925f95bceef8e9f3".to_string(),
                mainnet: "".to_string(),
            }),
        }
    }
}
