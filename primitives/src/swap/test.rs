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

#[macro_export]
macro_rules! assert_swap_work {
  ($market_order:expr, $limit_order:expr, $offer_base_amount:expr, $offer_quote_amount:expr, $market_pair:expr) => {{
    let one_percent = Permill::from_rational(1_u128, 100_u128);
    let one_millionth = Permill::from_rational(1_u128, 1_000_000_u128);
    let offer_quote_amount_1percent_more =
      $offer_quote_amount.saturating_add(one_percent * $offer_quote_amount);
    let offer_quote_amount_1percent_less =
      $offer_quote_amount.saturating_sub(one_percent * $offer_quote_amount);
    let offer_quote_amount_excceded_upperbound =
      offer_quote_amount_1percent_more.saturating_add(one_millionth * $offer_quote_amount);
    let offer_quote_amount_excceded_lowerbound =
      offer_quote_amount_1percent_less.saturating_sub(one_millionth * $offer_quote_amount);

    // should pass with exact numbers
    assert_eq!(
      $market_order.validate_slippage(
        &$limit_order,
        $offer_base_amount,
        $offer_quote_amount,
        &$market_pair,
      ),
      Ok(())
    );

    // should pass with the minimum seller acceptable price
    assert_eq!(
      $market_order.validate_slippage(
        &$limit_order,
        $offer_base_amount,
        offer_quote_amount_1percent_less,
        &$market_pair,
      ),
      Ok(())
    );

    // should pass with the maximum buyer acceptable price
    assert_eq!(
      $market_order.validate_slippage(
        &$limit_order,
        $offer_base_amount,
        offer_quote_amount_1percent_more,
        &$market_pair,
      ),
      Ok(())
    );

    // should fail with a price lower than the minimum seller acceptable price
    assert_eq!(
      $market_order.validate_slippage(
        &$limit_order,
        $offer_base_amount,
        offer_quote_amount_excceded_lowerbound,
        &$market_pair,
      ),
      if ($market_pair
        .is_selling(&$limit_order)
        .expect("market pair should be found in swap")
        && $limit_order.is_market_maker)
        || ($market_pair
          .is_selling(&$market_order)
          .expect("market pair should be found in swap")
          && $market_order.is_market_maker)
      {
        Err(SlippageError::OfferIsLessThanMarketMakerSwapLowerBound)
      } else {
        Err(SlippageError::OfferIsLessThanSwapLowerBound)
      }
    );

    // should fail with a price higher than maximum buyer acceptable price
    assert_eq!(
      $market_order.validate_slippage(
        &$limit_order,
        $offer_base_amount,
        offer_quote_amount_excceded_upperbound,
        &$market_pair,
      ),
      if (!$market_pair
        .is_selling(&$limit_order)
        .expect("market pair should be found in swap")
        && $limit_order.is_market_maker)
        || (!$market_pair
          .is_selling(&$market_order)
          .expect("market pair should be found in swap")
          && $market_order.is_market_maker)
      {
        Err(SlippageError::OfferIsGreaterThanMarketMakerSwapUpperBound)
      } else {
        Err(SlippageError::OfferIsGreaterThanSwapUpperBound)
      }
    );
  }};
}

#[cfg(test)]
mod tests {
  use crate::*;
  use std::str::FromStr;

  const ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
  const BOB: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

  fn build_test_swap(
    account_id: AccountId,
    swap_type: SwapType,
    token_from: CurrencyId,
    amount_from: Balance,
    token_to: CurrencyId,
    amount_to: Balance,
    slippage: Permill,
  ) -> Swap<AccountId, BlockNumber> {
    Swap {
      extrinsic_hash: Default::default(),
      account_id,
      is_market_maker: swap_type == SwapType::Limit,
      token_from,
      amount_from,
      amount_from_filled: 0,
      token_to,
      amount_to,
      amount_to_filled: 0,
      status: SwapStatus::Pending,
      swap_type,
      block_number: BlockNumber::from(1_u32),
      slippage,
    }
  }

  macro_rules! test {
    ($func:ident, $token_from:expr, $amount_from:expr, $token_to:expr, $amount_to:expr, $market_pair:expr) => {
      #[test]
      fn $func() {
        let one_percent = Permill::from_rational(1_u128, 100_u128);

        // Market Maker Limited Swap
        let limit_order = build_test_swap(
          AccountId::from_str(ALICE).unwrap(),
          SwapType::Limit,
          $token_to,
          $amount_to,
          $token_from,
          $amount_from,
          one_percent,
        );

        // Trader Market Swap
        let market_order = build_test_swap(
          AccountId::from_str(BOB).unwrap(),
          SwapType::Market,
          $token_from,
          $amount_from,
          $token_to,
          $amount_to,
          one_percent,
        );

        let offer_base_amount = if $token_from == $market_pair.base_asset {
          $amount_from
        } else {
          $amount_to
        };
        let offer_quote_amount = if $token_from == $market_pair.base_asset {
          $amount_to
        } else {
          $amount_from
        };

        assert_swap_work!(
          market_order,
          limit_order,
          offer_base_amount,
          offer_quote_amount,
          $market_pair
        )
      }
    };
  }

  // ATH_USDC
  test!(
    slippage_1_000_ath_to_10_usdc,
    Asset::AllTimeHigh.currency_id(),
    Asset::AllTimeHigh.saturating_mul(1_000),
    Asset::USDCoin.currency_id(),
    Asset::USDCoin.saturating_mul(10),
    MarketPair {
      base_asset: Asset::AllTimeHigh.currency_id(),
      quote_asset: Asset::USDCoin.currency_id(),
    }
  );
  test!(
    slippage_10_usdc_to_1_000_ath,
    Asset::USDCoin.currency_id(),
    Asset::USDCoin.saturating_mul(10),
    Asset::AllTimeHigh.currency_id(),
    Asset::AllTimeHigh.saturating_mul(1_000),
    MarketPair {
      base_asset: Asset::AllTimeHigh.currency_id(),
      quote_asset: Asset::USDCoin.currency_id(),
    }
  );

  // BTC_ETH
  test!(
    slippage_5_dot_9_eth_to_0_dot_9_btc,
    Asset::Ethereum.currency_id(),
    // 5.9 ETH
    Asset::Ethereum
      .saturating_mul(6)
      .saturating_sub(100_000_000_000_000_000),
    Asset::Bitcoin.currency_id(),
    // 0.9 BTC
    Asset::Bitcoin.saturating_mul(1).saturating_sub(10_000_000),
    MarketPair {
      base_asset: Asset::Bitcoin.currency_id(),
      quote_asset: Asset::Ethereum.currency_id(),
    }
  );
  test!(
    slippage_0_dot_9_btc_to_5_dot_9_eth,
    Asset::Bitcoin.currency_id(),
    // 0.9 BTC
    Asset::Bitcoin.saturating_mul(1).saturating_sub(10_000_000),
    Asset::Ethereum.currency_id(),
    // 5.9 ETH
    Asset::Ethereum
      .saturating_mul(6)
      .saturating_sub(100_000_000_000_000_000),
    MarketPair {
      base_asset: Asset::Bitcoin.currency_id(),
      quote_asset: Asset::Ethereum.currency_id(),
    }
  );

  // BTC_USDC
  test!(
    slippage_1_btc_to_30_000_usdc,
    Asset::Bitcoin.currency_id(),
    Asset::Bitcoin.saturating_mul(1),
    Asset::USDCoin.currency_id(),
    Asset::USDCoin.saturating_mul(30_000),
    MarketPair {
      base_asset: Asset::Bitcoin.currency_id(),
      quote_asset: Asset::USDCoin.currency_id(),
    }
  );
  test!(
    slippage_30_000_usdc_to_1_btc,
    Asset::USDCoin.currency_id(),
    Asset::USDCoin.saturating_mul(30_000),
    Asset::Bitcoin.currency_id(),
    Asset::Bitcoin.saturating_mul(1),
    MarketPair {
      base_asset: Asset::Bitcoin.currency_id(),
      quote_asset: Asset::USDCoin.currency_id(),
    }
  );

  // ETH_USDC
  test!(
    slippage_5_eth_to_2_900_usdc,
    Asset::Ethereum.currency_id(),
    Asset::Ethereum.saturating_mul(5),
    Asset::USDCoin.currency_id(),
    Asset::USDCoin.saturating_mul(2_900),
    MarketPair {
      base_asset: Asset::Ethereum.currency_id(),
      quote_asset: Asset::USDCoin.currency_id(),
    }
  );
  test!(
    slippage_2_900_usdc_to_5_eth,
    Asset::USDCoin.currency_id(),
    Asset::USDCoin.saturating_mul(2_900),
    Asset::Ethereum.currency_id(),
    Asset::Ethereum.saturating_mul(5),
    MarketPair {
      base_asset: Asset::Ethereum.currency_id(),
      quote_asset: Asset::USDCoin.currency_id(),
    }
  );

  // TDFY_BTC
  test!(
    slippage_1_btc_to_100_tdfy,
    Asset::Bitcoin.currency_id(),
    Asset::Bitcoin.saturating_mul(1),
    Asset::Tdfy.currency_id(),
    Asset::Tdfy.saturating_mul(100),
    MarketPair {
      base_asset: Asset::Tdfy.currency_id(),
      quote_asset: Asset::Bitcoin.currency_id(),
    }
  );
  test!(
    slippage_14_btc_to_2_900_000_tdfy,
    Asset::Bitcoin.currency_id(),
    Asset::Bitcoin.saturating_mul(14),
    Asset::Tdfy.currency_id(),
    Asset::Tdfy.saturating_mul(2_900_000),
    MarketPair {
      base_asset: Asset::Tdfy.currency_id(),
      quote_asset: Asset::Bitcoin.currency_id(),
    }
  );
  test!(
    slippage_100_tdfy_to_1_btc,
    Asset::Tdfy.currency_id(),
    Asset::Tdfy.saturating_mul(100),
    Asset::Bitcoin.currency_id(),
    Asset::Bitcoin.saturating_mul(1),
    MarketPair {
      base_asset: Asset::Tdfy.currency_id(),
      quote_asset: Asset::Bitcoin.currency_id(),
    }
  );

  // TDFY_ETH
  test!(
    slippage_5_eth_to_24_700_tdfy,
    Asset::Ethereum.currency_id(),
    Asset::Ethereum.saturating_mul(5),
    Asset::Tdfy.currency_id(),
    Asset::Tdfy.saturating_mul(24_700),
    MarketPair {
      base_asset: Asset::Tdfy.currency_id(),
      quote_asset: Asset::Ethereum.currency_id(),
    }
  );
  test!(
    slippage_24_700_tdfy_to_5_eth,
    Asset::Tdfy.currency_id(),
    Asset::Tdfy.saturating_mul(24_700),
    Asset::Ethereum.currency_id(),
    Asset::Ethereum.saturating_mul(5),
    MarketPair {
      base_asset: Asset::Tdfy.currency_id(),
      quote_asset: Asset::Ethereum.currency_id(),
    }
  );

  // TDFY_USDC
  test!(
    slippage_1_000_tdfy_to_2_900_usdc,
    Asset::Tdfy.currency_id(),
    Asset::Tdfy.saturating_mul(1_000),
    Asset::USDCoin.currency_id(),
    Asset::USDCoin.saturating_mul(2_900),
    MarketPair {
      base_asset: Asset::Tdfy.currency_id(),
      quote_asset: Asset::USDCoin.currency_id(),
    }
  );
  test!(
    slippage_2_900_usdc_to_1_000_tdfy,
    Asset::USDCoin.currency_id(),
    Asset::USDCoin.saturating_mul(2_900),
    Asset::Tdfy.currency_id(),
    Asset::Tdfy.saturating_mul(1_000),
    MarketPair {
      base_asset: Asset::Tdfy.currency_id(),
      quote_asset: Asset::USDCoin.currency_id(),
    }
  );
}
