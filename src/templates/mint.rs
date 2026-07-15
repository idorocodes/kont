 use crate::{KontInstruction, RawAccountMeta, constant::TOKEN_PROGRAM_ID, primitives::RawPubKey};

pub struct MintToTemplate {
    buffer: [u8; 105]
}

impl MintToTemplate {
    pub const fn new() -> Self {
        let mut buffer = [0u8; 105];
        buffer[0] = 7; // SPL Token MintTo discriminant
        Self { buffer }
    }

    #[inline(always)]
    pub fn set_mint(&mut self, key: &RawPubKey) {
        self.buffer[1..33].copy_from_slice(&key.0);
    }

    #[inline(always)]
    pub fn set_destination(&mut self, key: &RawPubKey) {
        self.buffer[33..65].copy_from_slice(&key.0);
    }

    #[inline(always)]
    pub fn set_mint_authority(&mut self, key: &RawPubKey) {
        self.buffer[65..97].copy_from_slice(&key.0);
    }

    #[inline(always)]
    pub fn set_amount(&mut self, amount: u64) {
        self.buffer[97..105].copy_from_slice(&amount.to_le_bytes());
    }

    pub fn to_kont_instruction(self) -> KontInstruction {
        let mut mint_data = [0u8; 32];
        mint_data.copy_from_slice(&self.buffer[1..33]);

        let mut destination_data = [0u8; 32];
        destination_data.copy_from_slice(&self.buffer[33..65]);

        // FIXED: Corrected index slice from 97..105 to 65..97
        let mut mint_au_data = [0u8; 32];
        mint_au_data.copy_from_slice(&self.buffer[65..97]);

        let mint_data_meta = RawAccountMeta {
            is_signer: false,
            pubkey: RawPubKey::new(&mint_data),
            is_writable: true
        };

        let destination_account_meta = RawAccountMeta {
            is_signer: false,
            pubkey: RawPubKey::new(&destination_data),
            is_writable: true
        };

        // FIXED: Authority is generally NOT writable in standard MintTo
        let mint_authority_meta = RawAccountMeta {
            is_signer: true,
            pubkey: RawPubKey::new(&mint_au_data),
            is_writable: false
        };

        let mut accounts = alloc::vec::Vec::new();
        accounts.push(mint_data_meta);
        accounts.push(destination_account_meta);
        accounts.push(mint_authority_meta);
        
        KontInstruction {
            program_id: TOKEN_PROGRAM_ID, 
            accounts: accounts.clone(),
            account_count: accounts.len(),
            data: self.buffer,
            data_len: self.buffer.len()
        }
    }
}


pub struct BurnTemplate {
    buffer: [u8; 105],
}

impl BurnTemplate {
    pub const fn new() -> Self {
        let mut buffer = [0u8; 105];
        buffer[0] = 8; // SPL Token Burn discriminant
        Self { buffer }
    }

    #[inline(always)]
    pub fn set_source(&mut self, key: &RawPubKey) {
        self.buffer[1..33].copy_from_slice(&key.0);
    }

    #[inline(always)]
    pub fn set_mint(&mut self, key: &RawPubKey) {
        self.buffer[33..65].copy_from_slice(&key.0);
    }

    #[inline(always)]
    pub fn set_authority(&mut self, key: &RawPubKey) {
        self.buffer[65..97].copy_from_slice(&key.0);
    }

    #[inline(always)]
    pub fn set_amount(&mut self, amount: u64) {
        self.buffer[97..105].copy_from_slice(&amount.to_le_bytes());
    }

    // FIXED: Fully implemented matching layout logic for BurnTemplate
    pub fn to_kont_instruction(self) -> KontInstruction {
        let mut source_data = [0u8; 32];
        source_data.copy_from_slice(&self.buffer[1..33]);

        let mut mint_data = [0u8; 32];
        mint_data.copy_from_slice(&self.buffer[33..65]);

        let mut authority_data = [0u8; 32];
        authority_data.copy_from_slice(&self.buffer[65..97]);

        let source_account_meta = RawAccountMeta {
            is_signer: false,
            pubkey: RawPubKey::new(&source_data),
            is_writable: true,
        };

        let mint_data_meta = RawAccountMeta {
            is_signer: false,
            pubkey: RawPubKey::new(&mint_data),
            is_writable: true,
        };

        let authority_meta = RawAccountMeta {
            is_signer: true,
            pubkey: RawPubKey::new(&authority_data),
            is_writable: false,
        };

        let mut accounts = alloc::vec::Vec::new();
        accounts.push(source_account_meta);
        accounts.push(mint_data_meta);
        accounts.push(authority_meta);

        KontInstruction {
            program_id: TOKEN_PROGRAM_ID,
            accounts: accounts.clone(),
            account_count: accounts.len(),
            data: self.buffer,
            data_len: self.buffer.len(),
        }
    }
}