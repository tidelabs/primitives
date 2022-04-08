// Copyright 2021-2022 Semantic Network Ltd.
// This file is part of tidefi-primitives.

// tidefi-primitives is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// tidefi-primitives is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with tidefi-primitives.  If not, see <http://www.gnu.org/licenses/>.

use crate::{AssetId, Balance, CurrencyId};
use codec::alloc::string::{String, ToString};
#[cfg(not(feature = "std"))]
use sp_arithmetic::traits::Saturating;

use tidefi_primitives_macro::assets;

#[cfg(feature = "std")]
use {
  serde::{Deserialize, Serialize},
  strum_macros::EnumIter,
};

pub enum Algo {
  SR25519,
  SECP256K1,
  WEB3,
}

#[assets]
pub enum Asset {
  #[asset::id = 1]
  #[asset::symbol = "TIFI"]
  #[asset::name = "Tidefi Token"]
  #[asset::decimals = 12]
  #[asset::algo = "SR25519"]
  #[asset::min_stake = 10_000_000_000_000]
  #[asset::max_stake = 500_000_000_000_000_000]
  Tifi,

  #[asset::id = 2]
  #[asset::symbol = "BTC"]
  #[asset::name = "Bitcoin"]
  #[asset::decimals = 8]
  #[asset::algo = "SECP256K1"]
  #[asset::unit = "satoshi"]
  #[asset::prefix = "₿"]
  #[asset::pot]
  #[asset::min_stake = 100]
  #[asset::max_stake = 500_000_000]
  Bitcoin,

  #[asset::id = 3]
  #[asset::symbol = "ETH"]
  #[asset::name = "Ethereum"]
  #[asset::decimals = 18]
  #[asset::algo = "WEB3"]
  #[asset::unit = "wei"]
  #[asset::prefix = "Ξ"]
  #[asset::min_stake = 100_000]
  #[asset::max_stake = 20_000_000_000_000_000_000]
  Ethereum,

  #[asset::id = 4]
  #[asset::symbol = "USDT"]
  #[asset::name = "Tether"]
  #[asset::decimals = 6]
  #[asset::algo = "WEB3"]
  #[asset::base_chain = "Ethereum"]
  #[asset::min_stake = 1_000_000]
  #[asset::max_stake = 100_000_000_000]
  Tether,

  #[asset::id = 5]
  #[asset::symbol = "USDC"]
  #[asset::name = "USD Coin"]
  #[asset::decimals = 6]
  #[asset::algo = "WEB3"]
  #[asset::base_chain = "Ethereum"]
  #[asset::min_stake = 1_000_000]
  #[asset::max_stake = 100_000_000_000]
  USDCoin,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_saturation_of_assets() {
    assert_eq!(Asset::Bitcoin.saturating_mul(10), 1_000_000_000);
    assert_eq!(Asset::Tifi.saturating_mul(912), 912_000_000_000_000);
    assert_eq!(
      Asset::USDCoin.saturating_mul(838_912_012),
      838_912_012_000_000
    );
  }
}
