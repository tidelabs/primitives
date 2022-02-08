#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::{
    prelude::{string::String, vec::Vec},
    TypeInfo,
};
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic, RuntimeDebug,
};

pub mod assets;

#[cfg(feature = "std")]
pub mod networks;

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
pub type DigestItem = generic::DigestItem;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// Block ID.
pub type BlockId = generic::BlockId<Block>;

/// Counter for the number of eras that have passed.
pub type EraIndex = u32;

/// Counter for the number of sessions that have passed.
pub type SessionIndex = u64;

/// Enum indicating the currency. Tide is the native token.
#[derive(Encode, Decode, TypeInfo, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Hash))]
pub enum CurrencyId {
    Tide,
    Wrapped(AssetId),
}
// default implementation but shouldn't be used please!
// it's required for the substrate storage
impl Default for CurrencyId {
    fn default() -> Self {
        CurrencyId::Tide
    }
}

/// Enum indicating status of the chain.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum StatusCode {
    /// Chain is fully operational.
    Running = 0,
    /// Chain is in maintenance and should be back soon.
    Maintenance = 1,
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::Running
    }
}

/// Withdrawal status.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum WithdrawalStatus {
    Pending,
    Cancelled,
    Approved,
    Rejected,
}

/// Withdrawal details.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone)]
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
    /// The block ID the withdrawal has been initialized.
    pub block_number: BlockNumber,
}

/// Swap status.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone)]
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

/// Swap details stored on-chain.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Swap<AccountId, BlockNumber> {
    /// Extrinsic hash of the initial swap request
    pub extrinsic_hash: [u8; 32],
    /// Account ID of the swap.
    pub account_id: AccountId,
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
    /// The block ID the swap request is in.
    pub block_number: BlockNumber,
}

/// Market maker swap confirmation.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone, Default)]
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

/// Stake details.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Stake<Balance, BlockNumber> {
    /// Block number the stake has started.
    /// We can compute the timestamp with `timestamp.now().at(BlockNumer)`
    /// Not saving the timestamp will same some space on-chain.
    pub initial_block: BlockNumber,
    /// Initial balance
    pub initial_balance: Balance,
    /// Principal balance (with accrued interest)
    pub principal: Balance,
    /// Duration of the stake
    pub duration: BlockNumber,
}

/// Currency metadata.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct CurrencyMetadata {
    /// Name of the currency
    pub name: Vec<u8>,
    /// Initial balance
    pub symbol: Vec<u8>,
    /// Number of decimals for the currency
    pub decimals: u8,
    /// Currency is frozen on chain (can't transfer)
    pub is_frozen: bool,
}

/// Information regarding the active era (era in used in session).
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ActiveEraInfo<BlockNumber> {
    /// Index of era.
    pub index: EraIndex,
    /// The block where the era started.
    pub start_block: Option<BlockNumber>,
    /// The session index where the era started.
    pub start_session_index: Option<SessionIndex>,
    /// The block where the last session ended.
    pub last_session_block: Option<BlockNumber>,
    /// Moment of start expressed as millisecond from `$UNIX_EPOCH`.
    ///
    /// Start can be none if start hasn't been set for the era yet,
    /// Start is set on the first on_finalize of the era to guarantee usage of `Time`.
    pub start: Option<u64>,
}

/// Information regarding a fee
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Fee {
    /// Total amount before fees
    pub amount: Balance,
    /// The fees at the moment of the transaction
    pub fee: Balance,
}

/// Currency balance.
#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct CurrencyBalance<Balance> {
    /// Available balance
    pub available: Balance,
    /// Reserved balance
    pub reserved: Balance,
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

pub mod pallet {
    use super::{Balance, CurrencyId, Fee, Hash, SessionIndex, Swap, Withdrawal};
    use scale_info::prelude::vec::Vec;
    use sp_runtime::DispatchError;
    /// Quorum traits to share with pallets.
    pub trait QuorumExt<AccountId, BlockNumber> {
        /// Get current Quorum status.
        fn is_quorum_enabled() -> bool;
        /// Add a new withdrawl to the queue.
        fn add_new_withdrawal_in_queue(
            account_id: AccountId,
            asset_id: CurrencyId,
            amount: Balance,
            external_address: Vec<u8>,
        ) -> (Hash, Withdrawal<AccountId, BlockNumber>);
    }

    /// Oracle traits to share with pallets.
    pub trait OracleExt<AccountId, BlockNumber> {
        /// Get current Quorum status.
        fn is_oracle_enabled() -> bool;
        /// Add a new swap to the queue.
        fn add_new_swap_in_queue(
            account_id: AccountId,
            asset_id_from: CurrencyId,
            amount_from: Balance,
            asset_id_to: CurrencyId,
            amount_to: Balance,
            block_number: BlockNumber,
            extrinsic_hash: [u8; 32],
        ) -> Result<(Hash, Swap<AccountId, BlockNumber>), DispatchError>;
        /// Cancel swap and release funds.
        fn remove_swap_from_queue(
            requester: AccountId,
            request_id: Hash,
        ) -> Result<(), DispatchError>;
    }

    pub trait SecurityExt<AccountId, BlockNumber> {
        /// Make sure the chain is running.
        fn is_chain_running() -> bool;
        /// Get the real block count processed when the chain was running. (Maintenance mode blocks are not calculated)
        fn get_current_block_count() -> BlockNumber;
        /// Generates a 256-bit unique hash from an `AccountId` and the
        /// internal (auto-incrementing) `Nonce` to prevent replay attacks
        fn get_unique_id(account_id: AccountId) -> Hash;
    }

    pub trait AssetRegistryExt {
        /// Make sure the currency exist and is enabled
        fn is_currency_enabled(currency_id: CurrencyId) -> bool;
    }

    pub trait FeesExt<AccountId> {
        /// Calculate the fee to be deposited into the central wallet
        /// You have to reduce the amount by the returned value manually and
        /// deposit the funds into the wallet
        fn calculate_swap_fees(currency_id: CurrencyId, total_amount_before_fees: Balance) -> Fee;
        /// Register a new swap fees associated with the account for the current era.
        /// A percentage of the network profits will be re-distributed to the account at the end of the era.
        fn register_swap_fees(
            account_id: AccountId,
            currency_id: CurrencyId,
            total_amount_before_fees: Balance,
        ) -> Fee;
        /// Get the account if of the central wallet to make deposit
        fn account_id() -> AccountId;
    }

    pub trait StakingExt<AccountId> {
        /// Triggered when a session end in the Fee pallet
        fn on_session_end(
            session_index: SessionIndex,
            session_trade_values: Vec<(CurrencyId, Balance)>,
        ) -> Result<(), DispatchError>;
        /// Get the staking account id where the funds are transfered
        fn account_id() -> AccountId;
    }

    pub trait WraprExt {}
}

#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Default)]
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
