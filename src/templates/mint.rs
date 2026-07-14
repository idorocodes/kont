use crate::primitives::RawPubKey;

pub struct  MintToTemplate{
    buffer : [u8;105]
}


impl MintToTemplate {
    
    pub const fn new() -> Self {
        let mut buffer = [0u8; 105];
        buffer[0] = 7; 
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
     
}


pub struct BurnTemplate {
    // 105 bytes of active instruction data layout
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

    
    // pub fn to_kont_instruction(self) -> KontInstruction {
    //     KontInstruction {
    //         program_id: RawPubKey([0u8; 32]), // Typically set to the SPL Token Program ID
    //         data: self.buffer.to_vec(),
    //     }
    // }
}