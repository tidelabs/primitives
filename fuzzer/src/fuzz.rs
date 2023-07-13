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

use sp_arithmetic::Permill;
use std::str::FromStr;
use tidefi_primitives::{
  assert_swap_work, assets::Asset, AccountId, BlockNumber, MarketPair, SlippageError, Swap, SwapStatus,
  SwapType,
};
const ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const BOB: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

fn run_one_swap(data: (u16, u16)) {
  let market_maker_amount_to_send = Asset::Tdfy.saturating_mul(data.0.into());
  let market_maker_amount_to_receive = Asset::Bitcoin.saturating_mul(data.1.into());
  let one_percent = Permill::from_rational(1_u128, 100_u128);

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

  #[cfg(not(fuzzing))]
  {
    println!(
      "Executing {} BTC -> {} TDFY",
      market_maker_amount_to_receive, market_maker_amount_to_send
    );
  }

  assert_swap_work!(
    market_order,
    limit_order,
    market_maker_amount_to_receive,
    market_maker_amount_to_send,
    MarketPair {
      base_asset: Asset::Tdfy.currency_id(),
      quote_asset: Asset::Bitcoin.currency_id(),
    }
  );
}

fn main() {
  #[cfg(fuzzing)]
  {
    loop {
      honggfuzz::fuzz!(|data: (u16, u16)| {
        run_one_swap(data);
      });
    }
  }
  #[cfg(not(fuzzing))]
  {
    use std::{env, process};
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
      println!("Please enter numbers; eg: primitives-fuzzer 1000 1");
      process::exit(1);
    }

    run_one_swap((
      args[1].parse().expect("valid digit as first arg"),
      args[2].parse().expect("valid digit as second arg"),
    ));
  }
}
