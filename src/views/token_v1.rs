 use crate::{errors::KontError, primitives::RawPubKey};


 // A solana token account at the core looks like this 
/* 
┌─────────────────────────────────────────────┐
│            Token Account (165 bytes)        │
├─────────────────────────────────────────────┤
│  mint: <which token>                        │
│  owner: <which wallet controls this>        │
│  amount: 500,000,000                        │  ← Balance (raw, before decimals)
│  delegate: <optional delegated authority>   │
│  state: Initialized                         │
│  is_native: false                           │
│  delegated_amount: 0                        │
│  close_authority: <optional>                │
└─────────────────────────────────────────────┘ */


// We just extract the part we need by reading the bytes.

pub struct TokenAccountView<'a> {
    data: &'a [u8],
}

impl<'a> TokenAccountView<'a> {
    pub fn try_from_slice(data: &'a [u8]) -> Result<Self, KontError> {
       
        if data.len() < 165 {
            return Err(KontError::InvalidBufferLength);
        }
        Ok(Self { data })
    }

    pub fn mint(&self) -> RawPubKey {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&self.data[0..32]);
        RawPubKey(bytes)
    }

    pub fn owner(&self) -> RawPubKey {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&self.data[32..64]);
        RawPubKey(bytes)
    }

    pub fn amount(&self) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.data[64..72]);
        u64::from_le_bytes(bytes)
    }

    pub fn delegate(&self) -> Option<RawPubKey> {
        
        let mut tag_bytes = [0u8; 4];
        tag_bytes.copy_from_slice(&self.data[72..76]);
        let tag = u32::from_le_bytes(tag_bytes);

     
        if tag == 0 {
            return None;
        }

        
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&self.data[76..108]);
        Some(RawPubKey(bytes))
    }

    pub fn is_frozen(&self) -> bool {
        // The state field is a single u8 at index 108.
        // 0 = Uninitialized, 1 = Initialized, 2 = Frozen
        self.data[108] == 2
    }
}