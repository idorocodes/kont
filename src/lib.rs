#![no_std]

pub mod errors;
pub mod primitives;

pub mod views;

pub mod templates;
use crate:: primitives::RawPubKey;
pub mod constant;

/// Maximum number of accounts any single Kont instruction template needs
/// (currently `TransferCheckedTemplate`, at 4 accounts).
pub const MAX_INSTRUCTION_ACCOUNTS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawAccountMeta {
    pub pubkey: RawPubKey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl Default for RawAccountMeta {
    /// An inert, all-zero placeholder used to pad unused slots in a
    /// fixed-size `[RawAccountMeta; N]` array. Never a valid account meta
    /// on its own — callers must only read the first `account_count` slots.
    fn default() -> Self {
        Self {
            pubkey: RawPubKey([0u8; 32]),
            is_signer: false,
            is_writable: false,
        }
    }
}


impl RawAccountMeta {
    /// Returns the raw 32-byte public key layout to match the bare account comparison.
    #[inline(always)]
    pub fn as_bytes(self) -> [u8; 32] {
        // Access the inner 32-byte fixed array inside the RawPubKey wrapper
        self.pubkey.0
    }

    /// Alias to prevent typos in your second test line if it calls `to_bytes` instead of `as_bytes`
    #[inline(always)]
    pub fn to_bytes(self) -> [u8; 32] {
        self.as_bytes()
    }
}

pub struct KontInstruction {
    pub program_id: RawPubKey,
    pub accounts: [RawAccountMeta; MAX_INSTRUCTION_ACCOUNTS],
    pub account_count: usize,
    pub data: [u8; 105],
    pub data_len: usize,
}
impl KontInstruction {
    #[inline(always)]
    pub fn program_id(&self) -> &RawPubKey {
        &self.program_id
    }

    #[inline(always)]
    pub fn account_count(&self) -> usize {
        self.account_count
    }

    /// Returns only the valid, populated account metas (ignores unused
    /// padding slots beyond `account_count`).
    #[inline(always)]
    pub fn accounts(&self) -> &[RawAccountMeta] {
        &self.accounts[..self.account_count]
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.data_len]
    }

    
}
