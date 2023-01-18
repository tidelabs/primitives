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
  ($func:ident, $token_from:expr, $amount_from:expr, $token_to:expr, $amount_to:expr) => {
    #[test]
    fn $func() {
      let one_percent = Permill::from_rational(1_u128, 100_u128);
      let two_percent = Permill::from_rational(2_u128, 100_u128);

      let limit_order = build_test_swap(
        AccountId::from_str(ALICE).unwrap(),
        SwapType::Limit,
        $token_to,
        $amount_to,
        $token_from,
        $amount_from,
        one_percent,
      );

      // The chain will allow 99 TDFY/BTC and 101 TDFY/BTC (1% slippage)
      let market_order = build_test_swap(
        AccountId::from_str(BOB).unwrap(),
        SwapType::Market,
        $token_from,
        $amount_from,
        $token_to,
        $amount_to,
        one_percent,
      );

      let market_maker_amount_to_receive = $amount_from;
      let market_maker_amount_to_send = $amount_to;

      let market_maker_amount_to_receive_more_1percent =
        market_maker_amount_to_receive.saturating_add(one_percent * market_maker_amount_to_receive);
      let market_maker_amount_to_receive_less_1percent =
        market_maker_amount_to_receive.saturating_sub(one_percent * market_maker_amount_to_receive);

      let market_maker_amount_to_receive_more_2percent =
        market_maker_amount_to_receive.saturating_add(two_percent * market_maker_amount_to_receive);
      let market_maker_amount_to_receive_less_2percent =
        market_maker_amount_to_receive.saturating_sub(two_percent * market_maker_amount_to_receive);

      let market_maker_amount_to_send_more_1percent =
        market_maker_amount_to_send.saturating_add(one_percent * market_maker_amount_to_send);
      let market_maker_amount_to_send_less_1percent =
        market_maker_amount_to_send.saturating_sub(one_percent * market_maker_amount_to_send);

      let market_maker_amount_to_send_more_2percent =
        market_maker_amount_to_send.saturating_add(two_percent * market_maker_amount_to_send);
      let market_maker_amount_to_send_less_2percent =
        market_maker_amount_to_send.saturating_sub(two_percent * market_maker_amount_to_send);

      // should pass with exact numbers
      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive,
          market_maker_amount_to_send
        ),
        Ok(())
      );

      // should pass with minimum numbers
      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive_more_1percent,
          market_maker_amount_to_send
        ),
        Ok(())
      );
      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive,
          market_maker_amount_to_send_more_1percent
        ),
        Ok(())
      );

      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive_less_1percent,
          market_maker_amount_to_send
        ),
        Ok(())
      );
      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive,
          market_maker_amount_to_send_less_1percent
        ),
        Ok(())
      );

      // should fails (2% slippage)
      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive_more_2percent,
          market_maker_amount_to_send
        ),
        Err(SlippageError::OfferIsLessThanSwapLowerBound)
      );
      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive,
          market_maker_amount_to_send_more_2percent
        ),
        Err(SlippageError::OfferIsGreaterThanSwapUpperBound)
      );

      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive,
          market_maker_amount_to_send_less_2percent
        ),
        Err(SlippageError::OfferIsLessThanSwapLowerBound)
      );
      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive_less_2percent,
          market_maker_amount_to_send
        ),
        Err(SlippageError::OfferIsGreaterThanSwapUpperBound)
      );

      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive_less_1percent.saturating_sub(100),
          market_maker_amount_to_send
        ),
        Err(SlippageError::OfferIsGreaterThanSwapUpperBound)
      );

      assert_eq!(
        market_order.validate_slippage(
          &limit_order,
          market_maker_amount_to_receive,
          market_maker_amount_to_send_less_1percent.saturating_sub(100)
        ),
        Err(SlippageError::OfferIsLessThanSwapLowerBound)
      );
    }
  };
}

test!(
  slippage_1_btc_to_100_tdfy,
  Asset::Bitcoin.currency_id(),
  Asset::Bitcoin.saturating_mul(1),
  Asset::Tdfy.currency_id(),
  Asset::Tdfy.saturating_mul(100)
);

test!(
  slippage_14_btc_to_2_900_000_tdfy,
  Asset::Bitcoin.currency_id(),
  Asset::Bitcoin.saturating_mul(14),
  Asset::Tdfy.currency_id(),
  Asset::Tdfy.saturating_mul(2_900_000)
);

test!(
  slippage_5_eth_to_24_700_tdfy,
  Asset::Ethereum.currency_id(),
  Asset::Ethereum.saturating_mul(5),
  Asset::Tdfy.currency_id(),
  Asset::Tdfy.saturating_mul(24_700)
);

test!(
  slippage_5_dot_9_eth_to_0_dot_9_btc,
  Asset::Ethereum.currency_id(),
  // 5.9 ETH
  Asset::Ethereum
    .saturating_mul(6)
    .saturating_add(100_000_000_000_000_000),
  Asset::Bitcoin.currency_id(),
  // 0.9 BTC
  Asset::Bitcoin.saturating_mul(1).saturating_sub(10_000_000)
);
