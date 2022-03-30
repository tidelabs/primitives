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

use crate::parse::Def;
pub fn expand(def: Def) -> proc_macro2::TokenStream {
  let enum_name = &def.item.ident;
  let enum_vis = &def.item.vis;

  let try_from_asset_id = def.assets.iter().map(|asset| {
    let symbol = &asset.symbol;
    let asset_id = &asset.inner.ident;
    quote::quote!(#symbol => Ok(#enum_name::#asset_id),)
  });

  let asset_id = def.assets.iter().map(|asset| {
    let symbol = &asset.symbol;
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #symbol,)
  });

  let symbols = def.assets.iter().map(|asset| {
    let asset_symbol = &asset.symbol.to_string();
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #asset_symbol.to_string(),)
  });

  let names = def.assets.iter().map(|asset| {
    let asset_name = &asset.name;
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #asset_name.to_string(),)
  });

  let decimals = def.assets.iter().map(|asset| {
    let decimal = &asset.decimals;
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #decimal,)
  });

  let algos = def.assets.iter().map(|asset| {
    let algo = &asset.algo;
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => Algo::#algo,)
  });

  let unit_names = def.assets.iter().map(|asset| {
    let algo = match &asset.unit {
      Some(algo) => {
        let algo_str = algo.to_string();
        quote::quote!(Some(#algo_str.to_string()))
      }
      None => quote::quote!(None),
    };
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #algo,)
  });

  let prefixes = def.assets.iter().map(|asset| {
    let prefix = match &asset.prefix {
      Some(prefix) => {
        let prefix_str = prefix.to_string();
        quote::quote!(Some(#prefix_str.to_string()))
      }
      None => quote::quote!(None),
    };
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #prefix,)
  });

  let base_chains = def.assets.iter().map(|asset| {
    let base_chain = match &asset.base_chain {
      Some(prefix) => {
        quote::quote!(Some(#enum_name::#prefix))
      }
      None => quote::quote!(None),
    };
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #base_chain,)
  });

  let min_stakes = def.assets.iter().map(|asset| {
    let min_stake = &asset.min_stake;
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #min_stake,)
  });

  let max_stakes = def.assets.iter().map(|asset| {
    let min_stake = &asset.max_stake;
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #min_stake,)
  });

  let all_pots = def.assets.iter().map(|asset| {
    let to_pot = asset.pot;
    let asset_id = &asset.inner.ident;
    quote::quote!(#enum_name::#asset_id => #to_pot,)
  });

  let all_consts = def.assets.iter().map(|asset| {
    let asset_id = &asset.id;
    let symbol = &asset.symbol;
    quote::quote!(pub const #symbol: AssetId = #asset_id;)
  });

  let try_from_asset_id2 = try_from_asset_id.clone();

  let all_items = def.assets.iter().map(|asset| asset.inner.ident.clone());

  quote::quote!(
    #(#all_consts)*

    #[derive(Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(EnumIter, Debug, Serialize, Deserialize, Hash))]
    #enum_vis enum #enum_name {
      #(#all_items),*
    }

    impl TryFrom<AssetId> for #enum_name {
      type Error = &'static str;
      fn try_from(asset: AssetId) -> Result<#enum_name, Self::Error> {
        match asset {
          #(#try_from_asset_id)*
          _ => Err("Invalid asset"),
        }
      }
    }

    impl TryFrom<CurrencyId> for Asset {
      type Error = &'static str;
      fn try_from(currency: CurrencyId) -> Result<#enum_name, Self::Error> {
        match currency {
          CurrencyId::Tide => Ok(#enum_name::Tide),
          CurrencyId::Wrapped(asset) => match asset {
            #(#try_from_asset_id2)*
            _ => Err("Invalid asset"),
          },
        }
      }
    }

    impl Asset {
      /// Get the `AssetId` used on-chain with the `pallet_assets`
      pub fn id(&self) -> AssetId {
        match self {
          #(#asset_id)*
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
          #(#symbols)*
        }
      }

      /// Return the asset name e.g.: Bitcoin
      pub fn name(&self) -> String {
        match self {
          #(#names)*
        }
      }

      /// Return the number of decimals. e.g.: `8` for `BTC`
      pub fn exponent(&self) -> u8 {
        match self {
          #(#decimals)*
        }
      }

      /// Return the algorythm for the coin
      pub fn algo(&self) -> Algo {
        match self {
          #(#algos)*
        }
      }

      /// Return the units name of the asset. e.g.: `wei`
      pub fn unit_name(&self) -> Option<String> {
        match self {
          #(#unit_names)*
        }
      }

      /// Return an optional prefix for the asset. e.g. `₿`
      pub fn prefix(&self) -> Option<String> {
        match self {
          #(#prefixes)*
        }
      }

      /// Based chain connected to the asset. (mainly used to identify wrapped tokens)
      pub fn base_chain(&self) -> Option<Asset> {
        match self {
          #(#base_chains)*
        }
      }

      /// Default minimum amount / stake, the value on-chain may differ.
      pub fn default_minimum_stake_amount(&self) -> Balance {
        match self {
          #(#min_stakes)*
        }
      }

      /// Default maximum amount / stake, the value on-chain may differ.
      pub fn default_maximum_stake_amount(&self) -> Balance {
        match self {
          #(#max_stakes)*
        }
      }

      /// Validate if these coins require a deposit to a second "pot" address
      pub fn to_pot(&self) -> bool {
        match self {
          #(#all_pots)*
        }
      }

      /// Saturating integer multiplication. Computes self * rhs, saturating at the numeric bounds instead of overflowing.
      /// By example, if you use `Asset::Bitcoin.saturating_mul(10)` it'll return `1_000_000_000`
      pub fn saturating_mul(&self, amount: Balance) -> Balance {
        amount.saturating_mul(10_u128.pow(self.exponent() as u32))
      }
    }
  )
}
