use crate::AssetId;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

pub const TIDE: AssetId = 1;
pub const BTC: AssetId = 2;
pub const ETH: AssetId = 3;
pub const USDT: AssetId = 4;
pub const USDC: AssetId = 5;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(EnumIter, Debug, Serialize, Deserialize, Hash))]
pub enum Asset {
    Tide,
    Bitcoin,
    Ethereum,
    Tether,
    USDCoin,
}

pub enum Algo {
    SR25519,
    SECP256K1,
    WEB3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "id")]
pub enum CurrencyId {
    Tide,
    Wrapped(AssetId),
}

impl Asset {
    pub fn id(&self) -> u32 {
        match self {
            Asset::Tide => TIDE,
            Asset::Bitcoin => BTC,
            Asset::Ethereum => ETH,
            Asset::Tether => USDT,
            Asset::USDCoin => USDC,
        }
    }
    pub fn currency_id(&self) -> CurrencyId {
        if self == &Asset::Tide {
            return CurrencyId::Tide;
        }
        CurrencyId::Wrapped(self.id())
    }
    pub fn symbol(&self) -> String {
        match self {
            Asset::Tide => "TIDE".to_string(),
            Asset::Bitcoin => "BTC".to_string(),
            Asset::Ethereum => "ETH".to_string(),
            Asset::USDCoin => "USDC".to_string(),
            Asset::Tether => "USDT".to_string(),
        }
    }
    pub fn exponent(&self) -> u8 {
        match self {
            Asset::Tide => 12,
            Asset::Bitcoin => 8,
            Asset::Ethereum => 18,
            Asset::USDCoin => 6,
            Asset::Tether => 6,
        }
    }
    pub fn algo(&self) -> Algo {
        match self {
            Asset::Tide => Algo::SR25519,
            Asset::Bitcoin => Algo::SECP256K1,
            Asset::Ethereum => Algo::WEB3,
            Asset::USDCoin => Algo::WEB3,
            Asset::Tether => Algo::WEB3,
        }
    }
    pub fn unit_name(&self) -> Option<String> {
        match self {
            Asset::Tide => None,
            Asset::Bitcoin => Some("satoshi".to_string()),
            Asset::Ethereum => Some("wei".to_string()),
            Asset::USDCoin => None,
            Asset::Tether => None,
        }
    }
    pub fn prefix(&self) -> Option<String> {
        match self {
            Asset::Tide => None,
            Asset::Bitcoin => Some("₿".to_string()),
            Asset::Ethereum => Some("Ξ".to_string()),
            Asset::USDCoin => None,
            Asset::Tether => None,
        }
    }
    pub fn base_chain(&self) -> Option<Asset> {
        match self {
            Asset::Tide => None,
            Asset::Bitcoin => None,
            Asset::Ethereum => None,
            Asset::USDCoin => Some(Asset::Ethereum),
            Asset::Tether => Some(Asset::Ethereum),
        }
    }
    // these coins require a deposit to a second "pot" address
    pub fn to_pot(&self) -> bool {
        if self == &Asset::Bitcoin {
            return true;
        }
        false
    }
}
