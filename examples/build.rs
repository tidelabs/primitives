use serde::Serialize;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use tidefi_primitives::{
    assets::{Asset, CurrencyId},
    networks::{Addresses, ChainIds, Network},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IToken {
    id: CurrencyId,
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

fn addresses_to_map(a: Addresses) -> Option<BTreeMap<String, String>> {
    let mut m = BTreeMap::new();
    m.insert("Devnet".to_string(), a.devnet);
    m.insert("Testnet".to_string(), a.testnet);
    m.insert("Mainnet".to_string(), a.mainnet);
    Some(m)
}

fn chainids_to_map(a: ChainIds) -> Option<BTreeMap<String, u32>> {
    let mut m = BTreeMap::new();
    m.insert("Devnet".to_string(), a.devnet);
    m.insert("Testnet".to_string(), a.testnet);
    m.insert("Mainnet".to_string(), a.mainnet);
    Some(m)
}

fn build_assets() {
    let mut tokens: Vec<IToken> = vec![];
    for asset in Asset::iter() {
        let mut token = IToken {
            id: asset.currency_id(),
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
        if let Some(ra) = asset.router() {
            token.router_address = addresses_to_map(ra);
        }
        if let Some(ma) = asset.multisig() {
            token.multisig_address = addresses_to_map(ma);
        }
        if let Some(aa) = asset.address() {
            token.asset_address = addresses_to_map(aa);
        }
        if let Some(cid) = asset.chain_id() {
            token.chain_id = chainids_to_map(cid);
        }
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
