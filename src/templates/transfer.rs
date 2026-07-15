use crate::{KontInstruction, RawAccountMeta, constant::TOKEN_PROGRAM_ID, primitives::RawPubKey};

pub struct TransferTemplate {
    accounts: [RawAccountMeta; 3],
    data: [u8; 9],
}
impl TransferTemplate {
    pub fn new() -> Self {
        Self {
            accounts: [
                RawAccountMeta {
                    pubkey: RawPubKey([0; 32]),
                    is_signer: false,
                    is_writable: true,
                },
                RawAccountMeta {
                    pubkey: RawPubKey([0; 32]),
                    is_signer: false,
                    is_writable: true,
                },
                RawAccountMeta {
                    pubkey: RawPubKey([0; 32]),
                    is_signer: true,
                    is_writable: false,
                },
            ],
            data: {
                let mut d = [0u8; 9];
                d[0] = 3; // Transfer discriminator
                d
            },
        }
    }
    pub fn set_source(&mut self, key: RawPubKey) {
        self.accounts[0].pubkey = key;
    }
    pub fn set_destination(&mut self, key: RawPubKey) {
        self.accounts[1].pubkey = key;
    }
    pub fn set_authority(&mut self, key: RawPubKey) {
        self.accounts[2].pubkey = key;
    }

    /// Copies the 8 little-endian bytes of the balance scalar into the terminal data slice.
    pub fn set_amount(&mut self, amount: u64) {
        let bytes = amount.to_le_bytes();
        // Assuming your terminal data slice is the final 8 bytes of the 114-byte layout (106..114)
        self.data[106..114].copy_from_slice(&bytes);
    }

    
    pub fn finish(self) -> KontInstruction {
    let mut buffer = [0u8; 105];

    buffer[..9].copy_from_slice(&self.data);

    let accounts = alloc::vec::Vec::from(self.accounts);

    KontInstruction {
        program_id: TOKEN_PROGRAM_ID,
        account_count: accounts.len(),
        accounts,
        data: buffer,
        data_len: 9,
    }
}
}

pub struct TransferCheckedTemplate {
    // 147-byte pre-allocated static stack buffer
    buffer: [u8; 147],
}

impl TransferCheckedTemplate {
    /// Allocation of a static 147-byte block on the stack,
    /// pre-stamped with instruction discriminant 12.
    pub const fn new() -> Self {
        let mut buffer = [0u8; 147];
        buffer[0] = 12; // Discriminant for TransferChecked
        Self { buffer }
    }

    /// Overwrites the specific byte region reserved for the sender.
    #[inline(always)]
    pub fn set_source(&mut self, key: &RawPubKey) {
        // Starts directly after the 1-byte instruction discriminant
        let offset = 1;
        self.buffer[offset..offset + 32].copy_from_slice(&key.0);
    }

    /// Injects the explicit token mint key into the instruction to fulfill safety requirements.
    #[inline(always)]
    pub fn set_mint(&mut self, key: &RawPubKey) {
        let offset = 1 + 32;
        self.buffer[offset..offset + 32].copy_from_slice(&key.0);
    }

    /// Overwrites the explicit recipient byte zone.
    #[inline(always)]
    pub fn set_destination(&mut self, key: &RawPubKey) {
        let offset = 1 + 32 + 32;
        self.buffer[offset..offset + 32].copy_from_slice(&key.0);
    }

    /// Sets the transaction signer field.
    #[inline(always)]
    pub fn set_authority(&mut self, key: &RawPubKey) {
        let offset = 1 + 32 + 32 + 32;
        self.buffer[offset..offset + 32].copy_from_slice(&key.0);
    }

    /// Blits the asset quantity parameter into the data block.
    #[inline(always)]
    pub fn set_amount(&mut self, amount: u64) {
        let offset = 1 + 32 + 32 + 32 + 32;
        self.buffer[offset..offset + 8].copy_from_slice(&amount.to_le_bytes());
    }

    /// Appends the matching token decimal value directly behind the amount integer.
    #[inline(always)]
    pub fn set_decimals(&mut self, decimals: u8) {
        let offset = 1 + 32 + 32 + 32 + 32 + 8;
        self.buffer[offset] = decimals;
    }

    // /// Seals the buffer sequence for transport handling.
    // pub fn to_kont_instruction(self) -> KontInstruction {
    //     KontInstruction::from_raw_parts(self.buffer)
    // }
}
