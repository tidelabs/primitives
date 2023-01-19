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

use sp_arithmetic::{traits::CheckedDiv, FixedPointNumber};

use crate::{
  Asset, Balance, CurrencyId, Decode, Deserialize, Encode, FixedU128, Hash, MaxEncodedLen, Permill,
  Serialize, TypeInfo,
};

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

#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum SlippageError {
  UnknownAsset,
  SlippageOverflow,
  ArithmeticError,
  OfferIsLessThanSwapLowerBound,
  OfferIsGreaterThanSwapUpperBound,
  OfferIsLessThanMarketMakerSwapLowerBound,
  OfferIsGreaterThanMarketMakerSwapUpperBound,
}

impl<AccountId: Clone, BlockNumber: Clone> Swap<AccountId, BlockNumber> {
  pub fn pay_per_token_lower_bond(&self) -> Result<FixedU128, SlippageError> {
    Ok(
      self
        .pay_per_token(
          |amount_to| {
            amount_to
              .checked_sub(self.slippage * amount_to)
              .ok_or(SlippageError::ArithmeticError)
              .unwrap_or(amount_to)
          },
          |amount_from| amount_from,
        )?
        .min(self.pay_per_token(
          |amount_to| amount_to,
          |amount_from| {
            amount_from
              .checked_add(self.slippage * amount_from)
              .ok_or(SlippageError::ArithmeticError)
              .unwrap_or(amount_from)
          },
        )?),
    )
  }

  pub fn pay_per_token_upper_bond(&self) -> Result<FixedU128, SlippageError> {
    Ok(
      self
        .pay_per_token(
          |amount_to| {
            amount_to
              .checked_add(self.slippage * amount_to)
              .ok_or(SlippageError::ArithmeticError)
              .unwrap_or(amount_to)
          },
          |amount_from| amount_from,
        )?
        .max(self.pay_per_token(
          |amount_to| amount_to,
          |amount_from| {
            amount_from
              .checked_sub(self.slippage * amount_from)
              .ok_or(SlippageError::ArithmeticError)
              .unwrap_or(amount_from)
          },
        )?),
    )
  }

  fn pay_per_token<FT, FF>(
    &self,
    amount_to_closure: FT,
    amount_from_closure: FF,
  ) -> Result<FixedU128, SlippageError>
  where
    FT: Fn(Balance) -> Balance,
    FF: Fn(Balance) -> Balance,
  {
    let token_to: Asset = self
      .token_to
      .try_into()
      .map_err(|_| SlippageError::UnknownAsset)?;

    let token_to_one_unit = token_to.saturating_mul(1);

    let token_from: Asset = self
      .token_from
      .try_into()
      .map_err(|_| SlippageError::UnknownAsset)?;
    let token_from_one_unit = token_from.saturating_mul(1);

    FixedU128::saturating_from_rational(amount_to_closure(self.amount_to), token_to_one_unit)
      .checked_div(&FixedU128::saturating_from_rational(
        amount_from_closure(self.amount_from),
        token_from_one_unit,
      ))
      .ok_or(SlippageError::SlippageOverflow)
  }

  fn validate_slippage_dry_run(
    &self,
    amount_to_receive: Balance,
    amount_to_send: Balance,
  ) -> Result<(), SlippageError> {
    let token_to: Asset = self
      .token_to
      .try_into()
      .map_err(|_| SlippageError::UnknownAsset)?;

    let token_to_one_unit = token_to.saturating_mul(1);

    let token_from: Asset = self
      .token_from
      .try_into()
      .map_err(|_| SlippageError::UnknownAsset)?;
    let token_from_one_unit = token_from.saturating_mul(1);

    let pay_per_token_lower_bond = self.pay_per_token_lower_bond()?;
    let pay_per_token_upper_bond = self.pay_per_token_upper_bond()?;

    let pay_per_token_offered =
      FixedU128::saturating_from_rational(amount_to_receive, token_to_one_unit)
        .checked_div(&FixedU128::saturating_from_rational(
          amount_to_send,
          token_from_one_unit,
        ))
        .ok_or(SlippageError::SlippageOverflow)?;

    // limit order can match with smaller price
    if self.swap_type != SwapType::Limit {
      if pay_per_token_lower_bond.gt(&pay_per_token_offered) {
        if self.is_market_maker {
          return Err(SlippageError::OfferIsLessThanMarketMakerSwapLowerBound);
        } else {
          return Err(SlippageError::OfferIsLessThanSwapLowerBound);
        }
      }
    }

    if pay_per_token_upper_bond.lt(&pay_per_token_offered) {
      if self.is_market_maker {
        return Err(SlippageError::OfferIsGreaterThanMarketMakerSwapUpperBound);
      } else {
        return Err(SlippageError::OfferIsGreaterThanSwapUpperBound);
      }
    }

    Ok(())
  }

  /// Validate slippage
  ///
  /// * `market_maker_swap` - Market maker (limit order) to test against.
  /// * `market_maker_amount_to_receive` - Expected amount the limit order will receive against `market_maker_amount_to_send`.
  /// * `market_maker_amount_to_send` - Expected amount the market order will receive against `market_maker_amount_to_receive`.
  pub fn validate_slippage(
    &self,
    market_maker_swap: &Swap<AccountId, BlockNumber>,
    market_maker_amount_to_receive: Balance,
    market_maker_amount_to_send: Balance,
  ) -> Result<(), SlippageError> {
    self.validate_slippage_dry_run(market_maker_amount_to_send, market_maker_amount_to_receive)?;

    market_maker_swap
      .validate_slippage_dry_run(market_maker_amount_to_receive, market_maker_amount_to_send)
  }
}
