use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AssetId {
    /// TIDE native currency of the chain
    TIDE,
    /// Generic enumerated asset
    /// Range 0 - 0x00000000FFFFFFFF (2^32)-1 is reserved for protected tokens
    /// the values under 1000 are used for ISO 4217 Numeric Curency codes
    ///
    /// FIXME: Build an official enum list, (don't need to be builtin the runtime)
    /// but can be consumed by the client, so the ID always match.
    Wrapr(u64),
}
