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

use crate::assets::Asset;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};
use strum_macros::EnumIter;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, EnumIter, Debug, Serialize, Deserialize, Hash)]
pub enum Network {
  Local,
  Devnet,
  Staging,
  Testnet,
  Mainnet,
}

pub type Addresses = BTreeMap<String, String>;

pub type ChainIds = BTreeMap<String, u32>;

pub type Enabled = BTreeMap<String, bool>;

impl FromStr for Network {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.to_lowercase() == "local" {
      return Ok(Network::Local);
    } else if s.to_lowercase() == "devnet" {
      return Ok(Network::Devnet);
    } else if s.to_lowercase() == "staging" {
      return Ok(Network::Staging);
    } else if s.to_lowercase() == "testnet" {
      return Ok(Network::Testnet);
    } else if s.to_lowercase() == "mainnet" {
      return Ok(Network::Mainnet);
    }
    Err("invalid network string".to_string())
  }
}

impl std::fmt::Display for Network {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Local => write!(f, "local"),
      Self::Devnet => write!(f, "devnet"),
      Self::Staging => write!(f, "staging"),
      Self::Testnet => write!(f, "testnet"),
      Self::Mainnet => write!(f, "mainnet"),
    }
  }
}

impl Asset {
  pub fn chain_id(&self) -> Option<ChainIds> {
    if self == &Asset::Ethereum {
      let mut ids = BTreeMap::new();
      ids.insert("local".to_string(), 1337);
      ids.insert("devnet".to_string(), 5);
      ids.insert("staging".to_string(), 5);
      ids.insert("testnet".to_string(), 5);
      ids.insert("mainnet".to_string(), 1);
      return Some(ids);
    }
    None
  }

  pub fn router(&self) -> Option<Addresses> {
    if self == &Asset::Ethereum {
      return Some(str_map(vec![
        ("local", "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"),
        ("devnet", "0x3b446e2eeb7a8171bd3a41452b22971e7d17aa80"),
        ("staging", "0xeed55a8e858d98371330d990358360ee36eeee6f"),
        ("testnet", "0x9d300e8b5991acabe98734dd0f0877d648cf11c6"),
        ("mainnet", "0x8f4b7bef83d6e2ef0d8bb23db8dbf7f9f2c69729"),
      ]));
    }
    None
  }

  pub fn multisig(&self) -> Option<Addresses> {
    if self == &Asset::Ethereum {
      return Some(str_map(vec![
        ("local", "0x5fc8d32690cc91d4c39d9d3abcbd16989f875707"),
        ("devnet", "0x22f3f691392c1d6c2c96b2333f08ccf0354f97b4"),
        ("staging", "0xe10ccf75d9bd5e2e64568c2e85c91e9005bb5dc5"),
        ("testnet", "0x05fadc62c72ded19387613664dc245e03c8da9b8"),
        ("mainnet", "0xff9d5585592507eff86d76cd9134a78e69786aa3"),
      ]));
    }
    None
  }

  pub fn address(&self) -> Option<Addresses> {
    if self == &Asset::Tether {
      return Some(str_map(vec![
        ("local", "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0"),
        ("devnet", "0x3fcbcc5df304cebfc3804dc8e70addf60cb05a1b"),
        ("staging", "0x348484e4a9a95dbd667398fe4f4fa6d4aaae4e18"),
        ("testnet", "0x74f8f1ba33f7def42ad29aba793c69497e512d2d"),
        ("mainnet", "0xdAC17F958D2ee523a2206206994597C13D831ec7"),
      ]));
    } else if self == &Asset::USDCoin {
      return Some(str_map(vec![
        ("local", "0xa513e6e4b8f2a923d98304ec87f64353c4d5c853"),
        ("devnet", "0x71819a038e02c521db8005936b3883cecfd886c0"),
        ("staging", "0x34c7391130c375fbbef15d8bc16907f001ad8cbd"),
        ("testnet", "0xbce2733e4b0eb15278ebb9f8496d3c638d1f43dd"),
        ("mainnet", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
      ]));
    }
    None
  }

  pub fn enabled(&self) -> Enabled {
    match self {
      Asset::Tdfy => bool_map(vec![
        ("local", true),
        ("devnet", true),
        ("staging", true),
        ("testnet", true),
        ("mainnet", true),
      ]),
      Asset::Bitcoin => bool_map(vec![
        ("local", true),
        ("devnet", true),
        ("staging", true),
        ("testnet", true),
        ("mainnet", true),
      ]),
      Asset::Ethereum => bool_map(vec![
        ("local", true),
        ("devnet", true),
        ("staging", true),
        ("testnet", true),
        ("mainnet", true),
      ]),
      Asset::Tether => bool_map(vec![
        ("local", true),
        ("devnet", true),
        ("staging", true),
        ("testnet", false),
        ("mainnet", false),
      ]),
      Asset::USDCoin => bool_map(vec![
        ("local", true),
        ("devnet", true),
        ("staging", true),
        ("testnet", true),
        ("mainnet", true),
      ]),
    }
  }
}

fn str_map(inp: Vec<(&str, &str)>) -> BTreeMap<String, String> {
  let mut r = BTreeMap::new();
  for (one, two) in inp {
    r.insert(one.to_string(), two.to_string());
  }
  r
}

fn bool_map(inp: Vec<(&str, bool)>) -> BTreeMap<String, bool> {
  let mut r = BTreeMap::new();
  for (one, two) in inp {
    r.insert(one.to_string(), two);
  }
  r
}
