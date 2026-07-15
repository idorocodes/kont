#![no_std]

pub mod errors;
pub mod primitives;

pub mod views;
extern crate alloc;

pub mod templates;
use crate::{constant::TOKEN_PROGRAM_ID, primitives::RawPubKey};
use alloc::vec::Vec;
pub mod constant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawAccountMeta {
    pub pubkey: RawPubKey,
    pub is_signer: bool,
    pub is_writable: bool,
}

pub struct KontInstruction {
    pub program_id: RawPubKey,
    pub accounts: Vec<RawAccountMeta>,
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

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.data_len]
    }

    
}
