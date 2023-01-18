use sp_arithmetic::Permill;
use std::str::FromStr;
use tidefi_primitives::{assets::Asset, AccountId, BlockNumber, Swap, SwapStatus, SwapType};
const ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const BOB: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

fn main() {
  let one_percent = Permill::from_rational(1_u128, 100_u128);

  loop {
    honggfuzz::fuzz!(|data: (u8, u8)| {
      let market_maker_amount_to_receive = Asset::Bitcoin.saturating_mul(data.1.into());
      let market_maker_amount_to_send = Asset::Tdfy.saturating_mul(data.0.into());

      let limit_order = Swap {
        extrinsic_hash: Default::default(),
        account_id: AccountId::from_str(ALICE).unwrap(),
        is_market_maker: true,
        token_from: Asset::Tdfy.currency_id(),
        amount_from: market_maker_amount_to_send,
        amount_from_filled: 0,
        token_to: Asset::Bitcoin.currency_id(),
        amount_to: market_maker_amount_to_receive,
        amount_to_filled: 0,
        status: SwapStatus::Pending,
        swap_type: SwapType::Limit,
        block_number: BlockNumber::from(1_u32),
        slippage: one_percent,
      };

      let market_order = Swap {
        extrinsic_hash: Default::default(),
        account_id: AccountId::from_str(BOB).unwrap(),
        is_market_maker: false,
        token_from: Asset::Bitcoin.currency_id(),
        amount_from: market_maker_amount_to_receive,
        amount_from_filled: 0,
        token_to: Asset::Tdfy.currency_id(),
        amount_to: market_maker_amount_to_send,
        amount_to_filled: 0,
        status: SwapStatus::Pending,
        swap_type: SwapType::Market,
        block_number: BlockNumber::from(1_u32),
        slippage: one_percent,
      };

      market_order
        .validate_slippage(
          &limit_order,
          market_maker_amount_to_receive,
          market_maker_amount_to_send,
        )
        .expect("valid price")
    });
  }
}
