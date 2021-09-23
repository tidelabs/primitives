#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::inherent::Vec;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic, RuntimeDebug,
};

pub mod assets;

#[cfg(feature = "std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them.
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Signed version of Balance
pub type Amount = i128;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Index = u32;

/// Represent a Wrapped Asset.
pub type AssetId = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// A timestamp: milliseconds since the unix epoch.
/// `u64` is enough to represent a duration of half a billion years, when the
/// time scale is milliseconds.
pub type Timestamp = u64;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// Block ID.
pub type BlockId = generic::BlockId<Block>;

/// The ID of a withdrawal request.
pub type RequestId = u32;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Hash))]
pub enum CurrencyId {
    Tide,
    Wrapped(AssetId),
}

/// Withdrawal status.
#[derive(Eq, PartialEq, Encode, Decode, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum WithdrawalStatus {
    Pending,
    Cancelled,
    Approved,
    Rejected,
}

/// Withdrawal details.
#[derive(Eq, PartialEq, Encode, Decode, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Withdrawal<AccountId, BlockNumber> {
    /// Status of the withdrawal.
    pub status: WithdrawalStatus,
    /// Account ID requesting the withdrawal.
    pub account_id: AccountId,
    /// The Asset ID to widthdraw.
    pub asset_id: CurrencyId,
    /// The amount of the asset to widthdraw.
    pub amount: Balance,
    /// The address on the AssetID chain where to send the funds.
    pub external_address: Vec<u8>,
    /// The block ID the withdrawal request is in.
    pub block_number: BlockNumber,
}

/// Trade status.
#[derive(Eq, PartialEq, Encode, Decode, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum TradeStatus {
    Pending,
    Cancelled,
    Approved,
    Rejected,
}

/// Trade details.
#[derive(Eq, PartialEq, Encode, Decode, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Trade<AccountId, BlockNumber> {
    /// Account ID of the trade.
    pub account_id: AccountId,
    /// Asset ID of the trade.
    pub token_from: CurrencyId,
    /// Amount from
    pub amount_from: Balance,
    /// Asset ID to the trade.
    pub token_to: CurrencyId,
    /// Amount to
    pub amount_to: Balance,
    /// Trade status
    pub status: TradeStatus,
    /// The block ID the trade request is in.
    pub block_number: BlockNumber,
}

pub mod pallet {
    use super::{Balance, CurrencyId, RequestId, Trade, Withdrawal};
    use frame_support::inherent::Vec;
    /// Quorum traits to share with pallets.
    pub trait QuorumExt<AccountId, BlockNumber> {
        /// Get current Quorum status.
        fn is_quorum_enabled() -> bool;

        /// Update Quorum status. All new request to the Quorum pallet will failed till the Quprum is restarted.
        fn set_quorum_status(is_enabled: bool);

        /// Add a new withdrawl request to the queue.
        fn add_new_withdrawal_in_queue(
            account_id: AccountId,
            asset_id: CurrencyId,
            amount: Balance,
            external_address: Vec<u8>,
        ) -> (RequestId, Withdrawal<AccountId, BlockNumber>);

        /// Add a new trade request to the queue.
        fn add_new_trade_in_queue(
            account_id: AccountId,
            asset_id_from: CurrencyId,
            amount_from: Balance,
            asset_id_to: CurrencyId,
            amount_to: Balance,
        ) -> (RequestId, Trade<AccountId, BlockNumber>);
    }

    pub trait WraprExt<AccountId> {}
}

/// App-specific crypto used for reporting equivocation/misbehavior in BABE and
/// GRANDPA. Any rewards for misbehavior reporting will be paid out to this
/// account.
pub mod report {

    use frame_system::offchain::AppCrypto;
    use sp_core::crypto::{key_types, KeyTypeId};

    use super::{Signature, Verify};

    /// Key type for the reporting module. Used for reporting BABE and GRANDPA
    /// equivocations.
    pub const KEY_TYPE: KeyTypeId = key_types::REPORTING;

    mod app {
        use sp_application_crypto::{app_crypto, sr25519};

        app_crypto!(sr25519, super::KEY_TYPE);
    }

    /// Identity of the equivocation/misbehavior reporter.
    pub type ReporterId = app::Public;

    /// An `AppCrypto` type to allow submitting signed transactions using the reporting
    /// application key as signer.
    pub struct ReporterAppCrypto;

    impl AppCrypto<<Signature as Verify>::Signer, Signature> for ReporterAppCrypto {
        type RuntimeAppPublic = ReporterId;
        type GenericPublic = sp_core::sr25519::Public;
        type GenericSignature = sp_core::sr25519::Signature;
    }
}

#[derive(Eq, PartialEq, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct BalanceInfo {
    #[cfg_attr(
        feature = "std",
        serde(bound(serialize = "Balance: std::fmt::Display"))
    )]
    #[cfg_attr(feature = "std", serde(serialize_with = "serialize_as_string"))]
    #[cfg_attr(
        feature = "std",
        serde(bound(deserialize = "Balance: std::str::FromStr"))
    )]
    #[cfg_attr(feature = "std", serde(deserialize_with = "deserialize_from_string"))]
    pub amount: Balance,
}

// Serializable Balance
#[cfg(feature = "std")]
fn serialize_as_string<S: Serializer, T: std::fmt::Display>(
    t: &T,
    serializer: S,
) -> core::result::Result<S::Ok, S::Error> {
    serializer.serialize_str(&t.to_string())
}

#[cfg(feature = "std")]
fn deserialize_from_string<'de, D: Deserializer<'de>, T: std::str::FromStr>(
    deserializer: D,
) -> core::result::Result<T, D::Error> {
    let s = String::deserialize(deserializer)?;
    s.parse::<T>()
        .map_err(|_| serde::de::Error::custom("Parse from string failed"))
}
