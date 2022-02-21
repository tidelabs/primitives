use crate::{AssetId, CurrencyId};
use codec::alloc::string::{String, ToString};

#[cfg(feature = "std")]
use {
    crate::Balance,
    serde::{Deserialize, Serialize},
    strum_macros::EnumIter,
};

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

impl Asset {
    /// Get the `AssetId` used on-chain with the pallet_assets
    pub fn id(&self) -> u32 {
        match self {
            Asset::Tide => TIDE,
            Asset::Bitcoin => BTC,
            Asset::Ethereum => ETH,
            Asset::Tether => USDT,
            Asset::USDCoin => USDC,
        }
    }

    /// Return the `CurrencyId` used by different pallets for Tidechain
    pub fn currency_id(&self) -> CurrencyId {
        if self == &Asset::Tide {
            return CurrencyId::Tide;
        }
        CurrencyId::Wrapped(self.id())
    }

    /// Return the symbol e.g.: BTC
    pub fn symbol(&self) -> String {
        match self {
            Asset::Tide => "TIDE".to_string(),
            Asset::Bitcoin => "BTC".to_string(),
            Asset::Ethereum => "ETH".to_string(),
            Asset::USDCoin => "USDC".to_string(),
            Asset::Tether => "USDT".to_string(),
        }
    }

    /// Return the asset name e.g.: Bitcoin
    pub fn name(&self) -> String {
        match self {
            Asset::Tide => "Tide".to_string(),
            Asset::Bitcoin => "Bitcoin".to_string(),
            Asset::Ethereum => "Ethereum".to_string(),
            Asset::USDCoin => "USD Coin".to_string(),
            Asset::Tether => "Tether".to_string(),
        }
    }

    /// Return the number of decimals. e.g.: `8` for `BTC`
    pub fn exponent(&self) -> u8 {
        match self {
            Asset::Tide => 12,
            Asset::Bitcoin => 8,
            Asset::Ethereum => 18,
            Asset::USDCoin => 6,
            Asset::Tether => 6,
        }
    }

    /// Return the algorythm for the coin
    pub fn algo(&self) -> Algo {
        match self {
            Asset::Tide => Algo::SR25519,
            Asset::Bitcoin => Algo::SECP256K1,
            Asset::Ethereum => Algo::WEB3,
            Asset::USDCoin => Algo::WEB3,
            Asset::Tether => Algo::WEB3,
        }
    }

    /// Return the units name of the asset. e.g.: `wei`
    pub fn unit_name(&self) -> Option<String> {
        match self {
            Asset::Tide => None,
            Asset::Bitcoin => Some("satoshi".to_string()),
            Asset::Ethereum => Some("wei".to_string()),
            Asset::USDCoin => None,
            Asset::Tether => None,
        }
    }

    /// Return an optional prefix for the asset. e.g. `₿`
    pub fn prefix(&self) -> Option<String> {
        match self {
            Asset::Tide => None,
            Asset::Bitcoin => Some("₿".to_string()),
            Asset::Ethereum => Some("Ξ".to_string()),
            Asset::USDCoin => None,
            Asset::Tether => None,
        }
    }

    /// Based chain connected to the asset. (mainly used to identify wrapped tokens)
    pub fn base_chain(&self) -> Option<Asset> {
        match self {
            Asset::Tide => None,
            Asset::Bitcoin => None,
            Asset::Ethereum => None,
            Asset::USDCoin => Some(Asset::Ethereum),
            Asset::Tether => Some(Asset::Ethereum),
        }
    }

    /// Default minimum amount / stake, the value on-chain may differ.
    pub fn default_minimum_stake_amount(&self) -> Balance {
        match self {
            // 10 tide
            Asset::Tide => 10_000_000_000_000,
            // 0.00000100 BTC
            Asset::Bitcoin => 100,
            // 0.000000000000100000 ETH
            Asset::Ethereum => 100_000,
            // 1
            Asset::USDCoin => 1_000_000,
            Asset::Tether => 1_000_000,
        }
    }

    /// Default maximum amount / stake, the value on-chain may differ.
    pub fn default_maximum_stake_amount(&self) -> Balance {
        match self {
            // 500k tide
            Asset::Tide => 500_000_000_000_000_000,
            // 5 btc
            Asset::Bitcoin => 500_000_000,
            // 20 eth
            Asset::Ethereum => 20_000_000_000_000_000_000,
            // 100k USD
            Asset::USDCoin => 100_000_000_000,
            Asset::Tether => 100_000_000_000,
        }
    }

    /// Validate if these coins require a deposit to a second "pot" address
    pub fn to_pot(&self) -> bool {
        if self == &Asset::Bitcoin {
            return true;
        }
        false
    }

    /// Saturating integer multiplication. Computes self * rhs, saturating at the numeric bounds instead of overflowing.
    /// By example, if you saturating_mul(10) with `BTC` it'll return `1_000_000_000`
    #[cfg(feature = "std")]
    pub fn saturating_mul(&self, amount: Balance) -> Balance {
        amount.saturating_mul(10_u128.pow(self.exponent() as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_saturation_of_assets() {
        assert_eq!(Asset::Bitcoin.saturating_mul(10), 1_000_000_000);
        assert_eq!(Asset::Tide.saturating_mul(912), 912_000_000_000_000);
        assert_eq!(
            Asset::USDCoin.saturating_mul(838_912_012),
            838_912_012_000_000
        );
    }
}
