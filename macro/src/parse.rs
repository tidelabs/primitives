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

use crate::utils;
use syn::spanned::Spanned;

mod keyword {
  syn::custom_keyword!(asset);
}

mod keyword_fn {
  syn::custom_keyword!(id);
  syn::custom_keyword!(symbol);
  syn::custom_keyword!(name);
  syn::custom_keyword!(decimals);
  syn::custom_keyword!(algo);
  syn::custom_keyword!(unit);
  syn::custom_keyword!(prefix);
  syn::custom_keyword!(pot);
  syn::custom_keyword!(base_chain);
  syn::custom_keyword!(min_stake);
  syn::custom_keyword!(max_stake);
}

#[derive(Debug)]
pub enum FnAttr {
  Id(u32, proc_macro2::Span),
  Symbol(syn::Ident, proc_macro2::Span),
  Name(String, proc_macro2::Span),
  Decimals(u8, proc_macro2::Span),
  Algo(syn::Ident, proc_macro2::Span),
  Unit(syn::Ident, proc_macro2::Span),
  Prefix(String, proc_macro2::Span),
  Pot(bool, proc_macro2::Span),
  BaseChain(syn::Ident, proc_macro2::Span),
  MinStake(u128, proc_macro2::Span),
  MaxStake(u128, proc_macro2::Span),
}

impl FnAttr {
  fn attr_span(&self) -> proc_macro2::Span {
    match self {
      Self::Symbol(_, span)
      | Self::Id(_, span)
      | Self::Name(_, span)
      | Self::Decimals(_, span)
      | Self::Algo(_, span)
      | Self::Unit(_, span)
      | Self::Prefix(_, span)
      | Self::Pot(_, span)
      | Self::BaseChain(_, span)
      | Self::MinStake(_, span)
      | Self::MaxStake(_, span) => *span,
    }
  }
}

impl syn::parse::Parse for FnAttr {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    input.parse::<syn::Token![#]>()?;
    let attr_span = input.span();
    let content;
    syn::bracketed!(content in input);
    content.parse::<keyword::asset>()?;
    content.parse::<syn::Token![::]>()?;

    let lookahead = content.lookahead1();
    if lookahead.peek(keyword_fn::symbol) {
      content.parse::<keyword_fn::symbol>()?;
      content.parse::<syn::Token![=]>()?;
      let renamed_prefix = content.parse::<syn::LitStr>()?;
      let new_ident = syn::parse_str::<syn::Ident>(&renamed_prefix.value()).map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", renamed_prefix.value());
        syn::Error::new(renamed_prefix.span(), msg)
      })?;

      Ok(Self::Symbol(new_ident, attr_span))
    } else if lookahead.peek(keyword_fn::name) {
      content.parse::<keyword_fn::name>()?;
      content.parse::<syn::Token![=]>()?;
      let name = content.parse::<syn::LitStr>()?.value();

      Ok(Self::Name(name, attr_span))
    } else if lookahead.peek(keyword_fn::decimals) {
      content.parse::<keyword_fn::decimals>()?;
      content.parse::<syn::Token![=]>()?;
      let new_ident = content.parse::<syn::LitInt>()?;

      let decimals = new_ident.base10_parse::<u8>().map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", new_ident.base10_digits());
        syn::Error::new(new_ident.span(), msg)
      })?;

      Ok(Self::Decimals(decimals, attr_span))
    } else if lookahead.peek(keyword_fn::id) {
      content.parse::<keyword_fn::id>()?;
      content.parse::<syn::Token![=]>()?;
      let new_ident = content.parse::<syn::LitInt>()?;

      let decimals = new_ident.base10_parse::<u32>().map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", new_ident.base10_digits());
        syn::Error::new(new_ident.span(), msg)
      })?;

      Ok(Self::Id(decimals, attr_span))
    } else if lookahead.peek(keyword_fn::algo) {
      content.parse::<keyword_fn::algo>()?;
      content.parse::<syn::Token![=]>()?;

      let renamed_prefix = content.parse::<syn::LitStr>()?;
      let new_ident = syn::parse_str::<syn::Ident>(&renamed_prefix.value()).map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", renamed_prefix.value());
        syn::Error::new(renamed_prefix.span(), msg)
      })?;

      Ok(Self::Algo(new_ident, attr_span))
    } else if lookahead.peek(keyword_fn::unit) {
      content.parse::<keyword_fn::unit>()?;
      content.parse::<syn::Token![=]>()?;

      let renamed_prefix = content.parse::<syn::LitStr>()?;
      let new_ident = syn::parse_str::<syn::Ident>(&renamed_prefix.value()).map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", renamed_prefix.value());
        syn::Error::new(renamed_prefix.span(), msg)
      })?;

      Ok(Self::Unit(new_ident, attr_span))
    } else if lookahead.peek(keyword_fn::prefix) {
      content.parse::<keyword_fn::prefix>()?;
      content.parse::<syn::Token![=]>()?;

      let prefix = content.parse::<syn::LitStr>()?.value();

      Ok(Self::Prefix(prefix, attr_span))
    } else if lookahead.peek(keyword_fn::pot) {
      content.parse::<keyword_fn::pot>()?;
      Ok(Self::Pot(true, attr_span))
    } else if lookahead.peek(keyword_fn::base_chain) {
      content.parse::<keyword_fn::base_chain>()?;
      content.parse::<syn::Token![=]>()?;

      let renamed_prefix = content.parse::<syn::LitStr>()?;
      let new_ident = syn::parse_str::<syn::Ident>(&renamed_prefix.value()).map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", renamed_prefix.value());
        syn::Error::new(renamed_prefix.span(), msg)
      })?;

      Ok(Self::BaseChain(new_ident, attr_span))
    } else if lookahead.peek(keyword_fn::min_stake) {
      content.parse::<keyword_fn::min_stake>()?;
      content.parse::<syn::Token![=]>()?;
      let new_ident = content.parse::<syn::LitInt>()?;

      let decimals = new_ident.base10_parse::<u128>().map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", new_ident.base10_digits());
        syn::Error::new(new_ident.span(), msg)
      })?;

      Ok(Self::MinStake(decimals, attr_span))
    } else if lookahead.peek(keyword_fn::max_stake) {
      content.parse::<keyword_fn::max_stake>()?;
      content.parse::<syn::Token![=]>()?;
      let new_ident = content.parse::<syn::LitInt>()?;

      let decimals = new_ident.base10_parse::<u128>().map_err(|_| {
        let msg = format!("`{}` is not a valid identifier", new_ident.base10_digits());
        syn::Error::new(new_ident.span(), msg)
      })?;

      Ok(Self::MaxStake(decimals, attr_span))
    } else {
      Err(lookahead.error())
    }
  }
}

struct FnAttrInfo {
  id: u32,
  symbol: syn::Ident,
  name: String,
  decimals: u8,
  algo: syn::Ident,
  unit: Option<syn::Ident>,
  prefix: Option<String>,
  pot: bool,
  base_chain: Option<syn::Ident>,
  min_stake: u128,
  max_stake: u128,
}

impl FnAttrInfo {
  fn from_attrs(attrs: Vec<FnAttr>, item_span: proc_macro2::Span) -> syn::Result<Self> {
    let mut id = None;
    let mut symbol = None;
    let mut name = None;
    let mut decimals = None;
    let mut algo = None;
    let mut unit = None;
    let mut prefix = None;
    let mut pot = false;
    let mut base_chain = None;
    let mut min_stake = None;
    let mut max_stake = None;

    for attr in attrs {
      match attr {
        FnAttr::Id(ident, ..) if id.is_none() => id = Some(ident),
        FnAttr::Symbol(ident, ..) if symbol.is_none() => symbol = Some(ident),
        FnAttr::Name(ident, ..) if name.is_none() => name = Some(ident),
        FnAttr::Decimals(found_rpc, ..) if decimals.is_none() => decimals = Some(found_rpc),
        FnAttr::Algo(found_const, ..) if algo.is_none() => algo = Some(found_const),
        FnAttr::Unit(found_const, ..) if unit.is_none() => unit = Some(found_const),
        FnAttr::Prefix(found_const, ..) if prefix.is_none() => prefix = Some(found_const),
        FnAttr::MinStake(found_const, ..) if min_stake.is_none() => min_stake = Some(found_const),
        FnAttr::MaxStake(found_const, ..) if max_stake.is_none() => max_stake = Some(found_const),
        FnAttr::Pot(found_const, ..) => pot = found_const,
        FnAttr::BaseChain(found_const, ..) if base_chain.is_none() => {
          base_chain = Some(found_const)
        }
        attr => {
          return Err(syn::Error::new(
            attr.attr_span(),
            "Invalid attribute: Duplicate attribute",
          ))
        }
      }
    }

    Ok(FnAttrInfo {
      id: id.ok_or_else(|| syn::Error::new(item_span, "Missing `#[asset::id]`"))?,
      symbol: symbol.ok_or_else(|| syn::Error::new(item_span, "Missing `#[asset::symbol]`"))?,
      name: name.ok_or_else(|| syn::Error::new(item_span, "Missing `#[asset::name]`"))?,
      decimals: decimals
        .ok_or_else(|| syn::Error::new(item_span, "Missing `#[asset::decimals]`"))?,
      algo: algo.ok_or_else(|| syn::Error::new(item_span, "Missing `#[asset::algo]`"))?,
      min_stake: min_stake
        .ok_or_else(|| syn::Error::new(item_span, "Missing `#[asset::min_stake]`"))?,
      max_stake: max_stake
        .ok_or_else(|| syn::Error::new(item_span, "Missing `#[asset::max_stake]`"))?,
      unit,
      prefix,
      pot,
      base_chain,
    })
  }
}

#[derive(Debug)]
pub struct Asset {
  pub id: u32,
  pub inner: syn::Variant,
  pub symbol: syn::Ident,
  pub name: String,
  pub decimals: u8,
  pub algo: syn::Ident,
  pub unit: Option<syn::Ident>,
  pub prefix: Option<String>,
  pub pot: bool,
  pub base_chain: Option<syn::Ident>,
  pub min_stake: u128,
  pub max_stake: u128,
}

impl Asset {
  pub fn try_from(item: &mut syn::Variant) -> syn::Result<Self> {
    let attrs: Vec<FnAttr> = utils::take_item_assets_attrs(&mut item.attrs.clone()).unwrap();

    let FnAttrInfo {
      id,
      symbol,
      name,
      decimals,
      algo,
      unit,
      prefix,
      pot,
      base_chain,
      min_stake,
      max_stake,
    } = FnAttrInfo::from_attrs(attrs, item.span()).unwrap();

    Ok(Asset {
      id,
      inner: item.clone(),
      symbol,
      name,
      decimals,
      algo,
      unit,
      prefix,
      pot,
      base_chain,
      min_stake,
      max_stake,
    })
  }
}

#[derive(Debug)]
pub struct Def {
  pub item: syn::ItemEnum,
  pub assets: Vec<Asset>,
}

impl Def {
  pub fn try_from(mut item: syn::ItemEnum) -> syn::Result<Self> {
    let assets = item
      .variants
      .iter_mut()
      .map(|asset_item| Asset::try_from(asset_item).unwrap())
      .collect();

    let def = Def { assets, item };

    Ok(def)
  }
}
