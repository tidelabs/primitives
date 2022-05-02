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

use serde::Serialize;
use std::{collections::BTreeMap, fs, path::PathBuf};
use structopt::StructOpt;
use strum::IntoEnumIterator;
use tidefi_primitives::{assets::Asset, networks::Network, AssetId, CurrencyId};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "id")]
pub enum BuildCurrencyId {
  Tdfy,
  Wrapped(AssetId),
}

impl From<CurrencyId> for BuildCurrencyId {
  fn from(id: CurrencyId) -> Self {
    match id {
      CurrencyId::Tdfy => Self::Tdfy,
      CurrencyId::Wrapped(id) => Self::Wrapped(id),
    }
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IToken {
  id: BuildCurrencyId,
  name: String,
  abbr: String,
  exponent: u8,
  #[serde(skip_serializing_if = "Option::is_none")]
  unit_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  symbol: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  base_chain: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  router_address: Option<BTreeMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  multisig_address: Option<BTreeMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  asset_address: Option<BTreeMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  chain_id: Option<BTreeMap<String, u32>>,
}

#[derive(Serialize)]
struct INetwork {
  name: String,
}

fn f(a: Asset) -> String {
  format!("{:?}", a)
}

fn build_assets(output: Option<PathBuf>) {
  let mut tokens: Vec<IToken> = vec![];
  for asset in Asset::iter() {
    let mut token = IToken {
      id: asset.currency_id().into(),
      name: f(asset.clone()),
      abbr: asset.symbol(),
      exponent: asset.exponent(),
      unit_name: asset.unit_name(),
      symbol: asset.prefix(),
      base_chain: None,
      router_address: None,
      multisig_address: None,
      asset_address: None,
      chain_id: None,
    };
    if let Some(bc) = asset.base_chain() {
      token.base_chain = Some(f(bc));
    }
    token.router_address = asset.router();
    token.multisig_address = asset.multisig();
    token.asset_address = asset.address();
    token.chain_id = asset.chain_id();
    tokens.push(token)
  }
  let tz = serde_json::to_string_pretty(&tokens).unwrap();
  let mut write_path = output.unwrap_or(
    std::env::current_exe()
      .expect("Unable to get current path")
      .parent()
      .expect("Unable to get current path")
      .to_path_buf(),
  );
  if !write_path.exists() {
    fs::create_dir_all(&write_path).expect("Unable to create directory");
  }
  write_path.push("assets.json");
  println!("Writing: {:?}", write_path);
  std::fs::write(write_path, tz).expect("Unable to write file");
}

fn build_networks(output: Option<PathBuf>) {
  let mut networks: Vec<INetwork> = vec![];
  for net in Network::iter() {
    networks.push(INetwork {
      name: format!("{:?}", net),
    })
  }
  let tz = serde_json::to_string_pretty(&networks).unwrap();
  let mut write_path = output.unwrap_or(
    std::env::current_exe()
      .expect("Unable to get current path")
      .parent()
      .expect("Unable to get current path")
      .to_path_buf(),
  );
  if !write_path.exists() {
    fs::create_dir_all(&write_path).expect("Unable to create directory");
  }
  write_path.push("networks.json");

  println!("Writing: {:?}", write_path);
  std::fs::write(write_path, tz).expect("Unable to write file");
}

/// Utilities for working with tidefi-primitives
#[derive(Debug, StructOpt)]
struct Opts {
  #[structopt(subcommand)]
  command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
  /// Generate `networks.json` and `assets.json` to be consumed
  /// by javascript and other tools.
  ///
  /// # Example (with custom output directory)
  ///
  /// `tidefi-primitives json -o ./dist`
  Json {
    /// the path where to write the `JSON` files
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
  },
}

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;
  let args = Opts::from_args();

  match args.command {
    Command::Json { output } => {
      build_assets(output.clone());
      build_networks(output);
      Ok(())
    }
  }
}
