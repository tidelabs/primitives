use serde::Serialize;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use tidefi_primitives::{assets::Asset, networks::Network, AssetId, CurrencyId};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "id")]
pub enum BuildCurrencyId {
    Tide,
    Wrapped(AssetId),
}

impl From<CurrencyId> for BuildCurrencyId {
    fn from(id: CurrencyId) -> Self {
        match id {
            CurrencyId::Tide => Self::Tide,
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

fn main() {
    build_assets();
    build_networks();
}

fn f(a: Asset) -> String {
    format!("{:?}", a)
}

fn build_assets() {
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
    std::fs::write("./dist/assets.json", tz).expect("Unable to write file");
}

fn build_networks() {
    let mut networks: Vec<INetwork> = vec![];
    for net in Network::iter() {
        networks.push(INetwork {
            name: format!("{:?}", net),
        })
    }
    let tz = serde_json::to_string_pretty(&networks).unwrap();
    std::fs::write("./dist/networks.json", tz).expect("Unable to write file");
}
