use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AssetId {
    /// TIDE native currency of the chain
    TIDE,
    /// Currency Code Iso 4217, example: ETH, BTC, USD, USDT etc..
    Wrapr(String),
}
