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

use crate::{
  AccountId, Asset, Balance, BlockNumber, CurrencyId, Decode, Encode, FixedU128, Hash,
  MaxEncodedLen, Permill, TypeInfo,
};
use codec::alloc::string::String;
use sp_arithmetic::{traits::CheckedDiv, FixedPointNumber};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Swap status.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum SwapStatus {
  /// Initial status
  Pending,
  /// Cancelled
  Cancelled,
  /// Partially filled by market makers
  PartiallyFilled,
  /// Completed (totally filled)
  Completed,
  /// Something went wrong the swap has been rejected
  Rejected,
}

/// Swap type
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum SwapType {
  /// Market swaps can partially fill, but are deleted immediately upon partial fill
  Market,
  /// Limit swaps can be partially filled and stay on chain
  Limit,
}

/// Market maker swap confirmation.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct SwapConfirmation {
  /// Request ID of the market maker swap request, used to fulfill this swap request.
  pub request_id: Hash,
  /// Amount of the source, should be formatted with the source currency, the market maker will receive this amount of asset.
  pub amount_to_receive: Balance,
  /// Amount of the destination, should be formatted with the destination currency, the market maker will send this amount of asset,
  /// and the swap will be filled with this amount. It may provide a partial or a complete fill.
  pub amount_to_send: Balance,
}

/// Swap extrinsic with swap fee details.
#[derive(Eq, PartialEq, TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct SwapExtrinsic {
  /// Signed extrinsic
  pub extrinsic: String,
  /// The estimated swap fee
  pub swap_fee: Balance,
  /// The currency the swap fees are taken
  pub swap_fee_currency: CurrencyId,
}

/// Swap details stored on-chain.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Swap<AccountId, BlockNumber> {
  /// Extrinsic hash of the initial swap request
  pub extrinsic_hash: [u8; 32],
  /// Account ID of the swap.
  pub account_id: AccountId,
  /// Determines if the swap has been created by an official market maker
  pub is_market_maker: bool,
  /// Asset ID of the swap.
  pub token_from: CurrencyId,
  /// Amount from
  pub amount_from: Balance,
  /// Amount from (currently filled -- if partial)
  pub amount_from_filled: Balance,
  /// Asset ID to the swap.
  pub token_to: CurrencyId,
  /// Amount to (requested)
  pub amount_to: Balance,
  /// Amount to (currently filled -- if partial)
  pub amount_to_filled: Balance,
  /// Swap status
  pub status: SwapStatus,
  /// Swap type
  pub swap_type: SwapType,
  /// The block number the swap request has been created
  pub block_number: BlockNumber,
  /// Slippage tolerance on the `amount_to`
  pub slippage: Permill,
}

/// Swap market pair details stored on-chain.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct MarketPair {
  /// Base asset of the swap market pair
  pub base_asset: CurrencyId,
  /// Quote asset of the swap market pair
  pub quote_asset: CurrencyId,
}

#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum SlippageError {
  UnknownAsset,
  UnknownAssetInMarketPair,
  SlippageOverflow,
  ArithmeticError,
  OfferIsLessThanSwapLowerBound,
  OfferIsGreaterThanSwapUpperBound,
  OfferIsLessThanMarketMakerSwapLowerBound,
  OfferIsGreaterThanMarketMakerSwapUpperBound,
  NoLowerBoundForBuyingPrice,
  NoUpperBoundForSellingPrice,
}

impl<AccountId: Clone, BlockNumber: Clone> Swap<AccountId, BlockNumber> {
  pub fn pay_per_token_lower_bond(
    &self,
    market_pair: &MarketPair,
  ) -> Result<FixedU128, SlippageError> {
    if self.token_from == market_pair.base_asset {
      // selling
      return Ok(self.pay_per_token(
        |base_amount| base_amount,
        |quote_amount| {
          quote_amount
            .checked_sub(self.slippage * quote_amount)
            .ok_or(SlippageError::ArithmeticError)
            .unwrap_or(quote_amount)
        },
        market_pair,
        self.amount_from,
        self.amount_to,
      )?);
    }

    Err(SlippageError::NoLowerBoundForBuyingPrice)
  }

  pub fn pay_per_token_upper_bond(
    &self,
    market_pair: &MarketPair,
  ) -> Result<FixedU128, SlippageError> {
    if self.token_to == market_pair.base_asset {
      // buying
      return Ok(self.pay_per_token(
        |base_amount| base_amount,
        |quote_amount| {
          quote_amount
            .checked_add(self.slippage * quote_amount)
            .ok_or(SlippageError::ArithmeticError)
            .unwrap_or(quote_amount)
        },
        market_pair,
        self.amount_to,
        self.amount_from,
      )?);
    }

    Err(SlippageError::NoUpperBoundForSellingPrice)
  }

  fn pay_per_token<FT, FF>(
    &self,
    base_amount_closure: FT,
    quote_amount_closure: FF,
    market_pair: &MarketPair,
    base_amount: Balance,
    quote_amount: Balance,
  ) -> Result<FixedU128, SlippageError>
  where
    FT: Fn(Balance) -> Balance,
    FF: Fn(Balance) -> Balance,
  {
    let base_asset: Asset = market_pair
      .base_asset
      .try_into()
      .map_err(|_| SlippageError::UnknownAsset)?;

    let base_token_one_unit = base_asset.saturating_mul(1);

    let quote_asset: Asset = market_pair
      .quote_asset
      .try_into()
      .map_err(|_| SlippageError::UnknownAsset)?;
    let quote_token_one_unit = quote_asset.saturating_mul(1);

    FixedU128::saturating_from_rational(quote_amount_closure(quote_amount), quote_token_one_unit)
      .checked_div(&FixedU128::saturating_from_rational(
        base_amount_closure(base_amount),
        base_token_one_unit,
      ))
      .ok_or(SlippageError::SlippageOverflow)
  }

  fn validate_slippage_dry_run(
    &self,
    price_offered: FixedU128,
    market_pair: &MarketPair,
  ) -> Result<(), SlippageError> {
    if self.token_to == market_pair.base_asset {
      // buyer should not accept a price greater than upper bound
      let pay_per_token_upper_bond = self.pay_per_token_upper_bond(market_pair)?;
      if price_offered.gt(&pay_per_token_upper_bond) {
        if self.is_market_maker {
          return Err(SlippageError::OfferIsGreaterThanMarketMakerSwapUpperBound);
        } else {
          return Err(SlippageError::OfferIsGreaterThanSwapUpperBound);
        }
      }
    } else {
      let pay_per_token_lower_bond = self.pay_per_token_lower_bond(market_pair)?;
      if price_offered.lt(&pay_per_token_lower_bond) {
        // seller should not accept a price smaller than lower bound
        if self.is_market_maker {
          return Err(SlippageError::OfferIsLessThanMarketMakerSwapLowerBound);
        } else {
          return Err(SlippageError::OfferIsLessThanSwapLowerBound);
        }
      }
    }

    Ok(())
  }

  /// Validate slippage
  ///
  /// * `market_maker_swap` - Market maker (limit order) to test against.
  /// * `market_maker_amount_to_receive` - Expected amount the limit order will receive against `market_maker_amount_to_send`.
  /// * `market_maker_amount_to_send` - Expected amount the market order will receive against `market_maker_amount_to_receive`.
  /// * `market_pair` - Market pair that this swap belongs to
  pub fn validate_slippage(
    &self,
    market_maker_swap: &Swap<AccountId, BlockNumber>,
    offered_base_amount: Balance,
    offered_quote_amount: Balance,
    market_pair: &MarketPair,
  ) -> Result<(), SlippageError> {
    let base_asset: Asset = market_pair
      .base_asset
      .try_into()
      .map_err(|_| SlippageError::UnknownAsset)?;
    let base_asset_one_unit = base_asset.saturating_mul(1);

    let quote_asset: Asset = market_pair
      .quote_asset
      .try_into()
      .map_err(|_| SlippageError::UnknownAsset)?;
    let quote_asset_one_unit = quote_asset.saturating_mul(1);

    let price_offered =
      FixedU128::saturating_from_rational(offered_quote_amount, quote_asset_one_unit)
        .checked_div(&FixedU128::saturating_from_rational(
          offered_base_amount,
          base_asset_one_unit,
        ))
        .ok_or(SlippageError::SlippageOverflow)?;

    self.validate_slippage_dry_run(price_offered, market_pair)?;

    market_maker_swap.validate_slippage_dry_run(price_offered, market_pair)
  }
}

impl MarketPair {
  pub fn is_selling(&self, swap: &Swap<AccountId, BlockNumber>) -> Result<bool, SlippageError> {
    if swap.token_from == self.base_asset {
      Ok(true)
    } else if swap.token_from == self.quote_asset {
      Ok(false)
    } else {
      Err(SlippageError::UnknownAssetInMarketPair)
    }
  }
}
