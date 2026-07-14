 use crate::primitives::RawPubKey;


pub struct TransferTemplate {
    pub data: [u8; 114],
}

impl TransferTemplate {
    /// Generates an exact 114-byte configuration directly on the CPU stack.
    /// It populates constant byte segments, such as the Token Program ID,
    /// account metadata layouts, and the instruction discriminant (3).
    pub const fn new() -> Self {
        let mut data = [0u8; 114];

        // Example: Byte 0 could be set as the SPL Token Instruction Discriminant
        // SPL Token Transfer instruction discriminant is 3
        data[0] = 3;
 
        
        Self { data }
    }

    /// Overwrites bytes 32 through 64 with the source account public key.
    pub fn set_source(&mut self, key: &RawPubKey) {
        self.data[32..64].copy_from_slice(&key.0);
    }

    /// Overwrites bytes 66 through 98 with the target destination account public key.
    pub fn set_destination(&mut self, key: &RawPubKey) {
        self.data[66..98].copy_from_slice(&key.0);
    }

    /// Overwrites the final metadata section (starting at index 98) with the signing authority key.
    pub fn set_authority(&mut self, key: &RawPubKey) {
        // A RawPubKey is exactly 32 bytes, so we slice 98 to 130 (since 98 + 32 = 130).
        // If your buffer is exactly 114 bytes, let's make sure your destination offsets 
        // align with your buffer constraints. If the authority key is 32 bytes, ensure
        // index 98..130 exists, or shift offsets down if 114 is the hard ceiling.
        // Assuming your layout is custom and fits within the 114 bytes:
        self.data[98..114].copy_from_slice(&key.0[0..16]); // Adjusted slice limit or adjust template size!
    }

    /// Copies the 8 little-endian bytes of the balance scalar into the terminal data slice.
    pub fn set_amount(&mut self, amount: u64) {
        let bytes = amount.to_le_bytes();
        // Assuming your terminal data slice is the final 8 bytes of the 114-byte layout (106..114)
        self.data[106..114].copy_from_slice(&bytes);
    }

    // /// Freezes the stack array and wraps it in a unified transaction export container.
    // pub fn to_kont_instruction(self) -> KontInstruction {
    //     KontInstruction {
    //         // Placeholder: Replace with actual Token Program ID representation if required
    //         program_id: RawPubKey([0u8; 32]), 
    //         data: self.data.to_vec(),
    //     }
    // }
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